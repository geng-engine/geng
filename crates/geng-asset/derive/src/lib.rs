#![recursion_limit = "128"]
#![allow(unused_imports)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro2::TokenStream;

#[proc_macro_derive(Load, attributes(load))]
pub fn derive_assets(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    match DeriveInput::from_derive_input(&input) {
        Ok(input) => input.derive().into(),
        Err(e) => e.write_errors().into(),
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(load))]
struct DeriveInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), Field>,
    #[darling(default)]
    serde: Option<String>,
    #[darling(default)]
    sequential: bool,
}

#[derive(FromField)]
#[darling(attributes(load))]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    path: Option<String>,
    #[darling(default)]
    ext: Option<String>,
    #[darling(default)]
    postprocess: Option<syn::Path>,
    #[darling(default, map = "parse_syn")]
    load_with: Option<syn::Expr>,
    #[darling(default, map = "parse_syn")]
    list: Option<syn::Expr>,
    #[darling(default)]
    listed_in: Option<String>,
    #[darling(default, rename = "if")]
    condition: Option<syn::Expr>,
    #[darling(default)]
    serde: bool,
}

fn parse_syn<T: syn::parse::Parse>(value: Option<String>) -> Option<T> {
    value.map(|s| syn::parse_str(&s).unwrap())
}

impl DeriveInput {
    pub fn derive(self) -> TokenStream {
        let Self {
            ident,
            generics,
            data,
            serde,
            sequential,
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        if let Some(ext) = serde {
            return quote! {
                impl #impl_generics geng::asset::Load #ty_generics for #ident #where_clause {
                    fn load(manager: &geng::asset::Manager, path: &std::path::Path) -> geng::asset::Future<Self> {
                        let path = path.to_owned();
                        async move {
                            Ok(batbox::file::load_detect(path).await?)
                        }.boxed_local()
                    }
                    const DEFAULT_EXT: Option<&'static str> = Some(#ext);
                }
            };
        }

        let data = data.take_struct().unwrap();
        let field_names = data
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                field
                    .ident
                    .as_ref()
                    .map(|ident| quote! { #ident })
                    .unwrap_or_else(|| {
                        let index = syn::Index::from(index);
                        quote! { #index }
                    })
            })
            .collect::<Vec<_>>();
        let field_loaders = data.fields.iter().map(|field| {
            if let Some(expr) = &field.load_with {
                return quote!(#expr);
            }
            let ident = field.ident.as_ref().unwrap();
            let ext = match &field.ext {
                Some(ext) => quote!(Some(#ext)),
                None => quote!(None::<&str>),
            };
            if field.serde {
                return match &field.path {
                    Some(path) => quote! {
                        batbox::file::load_detect(base_path.join(#path))
                    },
                    None => quote! {
                        batbox::file::load_detect(base_path.join(stringify!(#ident)), #ext)
                    },
                };
            }
            let list = match (&field.listed_in, &field.list) {
                (None, None) => None,
                (None, Some(range)) => Some(quote! {
                    (#range).map(|item| item.to_string())
                }),
                (Some(listed_in), None) => Some({
                    let base_path = match &field.path {
                        Some(_) => quote! { base_path },
                        None => quote! {
                            base_path.join(stringify!(#ident))
                        },
                    };
                    quote! {
                        file::load_detect::<Vec<String>>(
                            #base_path.join(#listed_in)
                        ).await?.into_iter()
                    }
                }),
                (Some(_), Some(_)) => panic!("Can't specify both list and listed_in"),
            };
            let mut loader = if let Some(list) = list {
                let loader = match &field.path {
                    Some(path) => quote! {
                        manager.load(base_path.join(#path.replace("*", &item)))
                    },
                    None => quote! {
                        manager.load_ext(base_path.join(stringify!(#ident)).join(item), #ext)
                    },
                };
                quote! {
                    ::geng::prelude::futures::future::try_join_all((#list).map(|item| { #loader }))
                }
            } else {
                match &field.path {
                    Some(path) => quote! {
                       manager.load(base_path.join(#path))
                    },
                    None => quote! {
                        manager.load_ext(base_path.join(stringify!(#ident)), #ext)
                    },
                }
            };
            if let Some(postprocess) = &field.postprocess {
                loader = quote! {
                    #loader.map(|result| {
                        result.map(|mut asset| {
                            #postprocess(&mut asset);
                            asset
                        })
                    })
                };
            }
            loader
        });
        let field_loaders = data
            .fields
            .iter()
            .zip(field_loaders)
            .map(|(field, loader)| {
                let loader = if let Some(expr) = &field.condition {
                    quote! {
                        async {
                            Ok::<_, anyhow::Error>(if #expr {
                                Some(#loader.await?)
                            } else {
                                None
                            })
                        }
                    }
                } else {
                    loader
                };
                let ty = &field.ty;
                quote! {
                    async {
                        let value = #loader.await?;
                        Ok::<#ty, anyhow::Error>(value)
                    }
                }
            });
        let load_fields = if sequential {
            quote! {
                #(
                    let #field_names = anyhow::Context::context(
                        #field_loaders.await,
                        concat!("Failed to load ", stringify!(#field_names)),
                    )?;
                )*
            }
        } else {
            quote! {
                #(let #field_names = #field_loaders;)*
                let (#(#field_names,)*) = ::geng::prelude::futures::join!(#(#field_names,)*);
                #(
                    let #field_names = anyhow::Context::context(
                        #field_names,
                        concat!("Failed to load ", stringify!(#field_names)),
                    )?;
                )*
            }
        };
        quote! {
            impl geng::asset::Load for #ident
                /* where #(#field_constraints),* */ {
                fn load(manager: &geng::asset::Manager, base_path: &std::path::Path) -> geng::asset::Future<Self> {
                    let manager = manager.clone();
                    let base_path = base_path.to_owned();
                    Box::pin(async move {
                        #load_fields
                        Ok(Self {
                            #(#field_names,)*
                        })
                    })
                }
                const DEFAULT_EXT: Option<&'static str> = None;
            }
        }
    }
}
