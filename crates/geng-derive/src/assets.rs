use super::*;

#[derive(FromDeriveInput)]
#[darling(attributes(asset))]
pub struct DeriveInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), Field>,
    #[darling(default)]
    json: bool,
    #[darling(default)]
    sequential: bool,
}

#[derive(FromField)]
#[darling(attributes(asset))]
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
    range: Option<syn::Expr>,
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
            json,
            sequential,
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        if json {
            return quote! {
                impl #impl_generics geng::LoadAsset #ty_generics for #ident #where_clause {
                    fn load(geng: &Geng, path: &std::path::Path) -> geng::AssetFuture<Self> {
                        let json = geng.load_asset::<String>(path);
                        async move {
                            let json = json.await?;
                            Ok(serde_json::from_str(&json)?)
                        }.boxed_local()
                    }
                    const DEFAULT_EXT: Option<&'static str> = Some("json");
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
            let ext = match &field.ext {
                Some(ext) => quote!(Some(#ext)),
                None => quote!(None),
            };
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let mut loader = if let Some(range) = &field.range {
                let path = field
                    .path
                    .as_ref()
                    .expect("Path needs to be specified for ranged assets");
                quote! {
                    futures::future::try_join_all((#range).map(|i| {
                        geng.load_asset(base_path.join(#path.replace("*", &i.to_string())))
                    }))
                }
            } else {
                let path = match &field.path {
                    Some(path) => quote! { #path },
                    None => quote! {{
                        let mut path = stringify!(#ident).to_owned();
                        if let Some(ext) = #ext.or(<#ty as geng::LoadAsset>::DEFAULT_EXT) {
                            path.push('.');
                            path.push_str(ext);
                        }
                        path
                    }},
                };
                quote! {
                    geng.load_asset::<#ty>(base_path.join(#path))
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
                let (#(#field_names,)*) = futures::join!(#(#field_loaders,)*);
                #(
                    let #field_names = anyhow::Context::context(
                        #field_names,
                        concat!("Failed to load ", stringify!(#field_names)),
                    )?;
                )*
            }
        };
        quote! {
            impl geng::LoadAsset for #ident
                /* where #(#field_constraints),* */ {
                fn load(geng: &geng::Geng, base_path: &std::path::Path) -> geng::AssetFuture<Self> {
                    let geng = geng.clone();
                    let base_path = base_path.to_owned();
                    Box::pin(async move {
                        let geng = &geng;
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
