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
            let field_names_copy = field_names.clone();
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
                                        }
                                    }
                                }
                            }
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
                            futures::future::join_all((#range).map(|i| {
                                geng::LoadAsset::load(geng, &format!("{}/{}", path, #path.replace("*", &i.to_string())))
                            })).map(|results| results.into_iter().collect::<Result<#ty, anyhow::Error>>()).boxed_local()
                        }
                    } else {
                        let path = match path {
                            Some(path) => quote! { #path },
                            None => quote! {{
                                let mut path = stringify!(#name).to_owned();
                                if let Some(ext) = <#ty>::default_ext() {
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
            let future_name = syn::Ident::new(
                &format!("{}Future", input_type),
                proc_macro2::Span::call_site(),
            );

            let expanded = quote! {
                pub struct #future_name {
                    #(#field_names: std::pin::Pin<Box<geng::prelude::future::MaybeDone<geng::AssetFuture<#field_tys>>>>,)*
                }

                impl std::future::Future for #future_name {
                    type Output = Result<#input_type, anyhow::Error>;
                    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context) -> std::task::Poll<Self::Output> {
                        let mut all_done = true;
                        #(all_done &= geng::prelude::Future::poll(self.#field_names.as_mut(), cx).is_ready();)*
                        if all_done {
                            #(
                                let #field_names = match self.#field_names_copy.as_mut().take_output().unwrap() {
                                    Ok(value) => value,
                                    Err(e) => return std::task::Poll::Ready(Err(e)),
                                };
                            )*
                            std::task::Poll::Ready(Ok(#input_type {
                                #(#field_names,)*
                            }))
                        } else {
                            std::task::Poll::Pending
                        }
                    }
                }

                impl geng::LoadAsset for #input_type
                    /*where #(#field_constraints),**/ {
                    fn load(geng: &Rc<Geng>, base_path: &str) -> geng::AssetFuture<Self> {
                        geng::prelude::future::FutureExt::boxed_local(#future_name {
                            #(#field_names: std::pin::Pin::new(Box::new(geng::prelude::future::maybe_done(#field_loaders))),)*
                        })
                    }
                    fn default_ext() -> Option<&'static str> {
                        None
                    }
                }
            };
            expanded.into()
        }
        _ => panic!("geng::Assets can only be derived by structs"),
    }
}
