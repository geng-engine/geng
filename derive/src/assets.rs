use super::*;

pub fn derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();
    let input_type = &ast.ident;
    // let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let field_tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();
            let field_tys = &field_tys;
            let field_names: Vec<_> = fields
                .iter()
                .map(|field| field.ident.as_ref().unwrap())
                .collect();
            let field_names = &field_names;
            let field_attrs: Vec<_> = fields
                .iter()
                .map(|field| {
                    let mut path = None;
                    let mut range = None;
                    for attr in &field.attrs {
                        if let Ok(syn::Meta::List(syn::MetaList {
                            path: ref meta_path,
                            ref nested,
                            ..
                        })) = attr.parse_meta()
                        {
                            if meta_path.is_ident("asset") {
                                for inner in nested {
                                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(
                                        syn::MetaNameValue {
                                            path: ref meta_path,
                                            ref lit,
                                            ..
                                        },
                                    )) = *inner
                                    {
                                        if meta_path.is_ident("path") {
                                            assert!(path.is_none(), "Multiple paths for an asset");
                                            path = Some(lit.clone());
                                        } else if meta_path.is_ident("range") {
                                            assert!(
                                                range.is_none(),
                                                "Multiple ranges for an asset"
                                            );
                                            range = Some(lit.clone());
                                        } else {
                                            panic!("Failed to parse asset attr");
                                        }
                                    } else {
                                        panic!("Failed to parse asset attr");
                                    }
                                }
                            }
                        } else {
                            panic!("Failed to parse meta")
                        }
                    }
                    (path, range)
                })
                .collect();
            let field_placeholders: Vec<_> = fields
                .iter()
                .map(|field| {
                    let mut placeholder = None;
                    for attr in &field.attrs {
                        if let Ok(syn::Meta::NameValue(syn::MetaNameValue {
                            path: ref meta_path,
                            lit: syn::Lit::Str(ref s),
                            ..
                        })) = attr.parse_meta()
                        {
                            if meta_path.is_ident("placeholder") {
                                placeholder = Some(s.value());
                            }
                        }
                    }
                    placeholder.map(|s| syn::parse_str::<syn::Expr>(&s).unwrap())
                })
                .collect();

            let field_loaders = izip!(
                field_names.iter(),
                field_tys.iter(),
                field_attrs.iter(),
                field_placeholders.iter()
            )
            .map(|(name, ty, (path, range), placeholder)| match placeholder {
                Some(_placeholder) => panic!("Lazy assets removed"),
                None => {
                    if let Some(syn::Lit::Str(ref range)) = range {
                        let path = path.as_ref().expect("Path needs to be specified for ranged assets");
                        let range = range.parse::<syn::ExprRange>().expect("Failed to parse range");
                        quote! {
                            futures::future::try_join_all((#range).map(|i| {
                                geng::LoadAsset::load(geng, &format!("{}/{}", base_path, #path.replace("*", &i.to_string())))
                            }))
                        }
                    } else {
                        let path = match path {
                            Some(path) => quote! { #path },
                            None => quote! {{
                                let mut path = stringify!(#name).to_owned();
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
                    }
                }
            });

            let expanded = quote! {
                impl geng::LoadAsset for #input_type
                    /* where #(#field_constraints),* */ {
                    fn load(geng: &Rc<Geng>, base_path: &str) -> geng::AssetFuture<Self> {
                        let (#(#field_names,)*) = (#(#field_loaders,)*);
                        Box::pin(async move {
                            let (#(#field_names,)*) = futures::try_join!(#(#field_names,)*)?;
                            Ok(Self {
                                #(#field_names,)*
                            })
                        })
                    }
                    const DEFAULT_EXT: Option<&'static str> = None;
                }
            };
            expanded.into()
        }
        _ => panic!("geng::Assets can only be derived by structs"),
    }
}
