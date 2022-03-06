use super::*;

#[derive(FromDeriveInput)]
#[darling(attributes(asset), supports(struct_named))]
pub struct DeriveInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), Field>,
    #[darling(default)]
    json: bool,
}

#[derive(FromField)]
#[darling(attributes(asset))]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    path: Option<String>,
    #[darling(default)]
    load_with: Option<syn::Expr>,
    #[darling(default)]
    range: Option<syn::Expr>,
}

impl DeriveInput {
    pub fn derive(self) -> TokenStream {
        let Self {
            ident,
            generics,
            data,
            json,
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        if json {
            return quote! {
                impl #impl_generics geng::LoadAsset #ty_generics for #ident #where_clause {
                    fn load(geng: &Geng, path: &str) -> geng::AssetFuture<Self> {
                        let json = <String as geng::LoadAsset>::load(geng, path);
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
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            if let Some(range) = &field.range {
                let path = field.path.as_ref().expect("Path needs to be specified for ranged assets");
                return quote! {
                    futures::future::try_join_all((#range).map(|i| {
                        geng::LoadAsset::load(geng, &format!("{}/{}", base_path, #path.replace("*", &i.to_string())))
                    }))
                };
            }
            let path = match &field.path {
                Some(path) => quote! { #path },
                None => quote! {{
                    let mut path = stringify!(#ident).to_owned();
                    if let Some(ext) = <#ty as geng::LoadAsset>::DEFAULT_EXT {
                        path.push('.');
                        path.push_str(ext);
                    }
                    path
                }},
            };
            quote! {
                <#ty as geng::LoadAsset>::load(geng, &format!("{}/{}", base_path, #path))
            }
        });
        quote! {
            impl geng::LoadAsset for #ident
                /* where #(#field_constraints),* */ {
                fn load(geng: &geng::Geng, base_path: &str) -> geng::AssetFuture<Self> {
                    let geng = geng.clone();
                    let base_path = base_path.to_owned();
                    Box::pin(async move {
                        let geng = &geng;
                        let base_path = base_path.as_str();
                        #(
                            let #field_names = anyhow::Context::context(
                                #field_loaders.await,
                                concat!("Failed to load ", stringify!(#field_names)),
                            )?;
                        )*
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
