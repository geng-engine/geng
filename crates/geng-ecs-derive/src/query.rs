use super::*;

pub fn derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();
    let input_type = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let fetch_lifetime = match ast.generics.params.iter().next() {
        Some(syn::GenericParam::Lifetime(syn::LifetimeDef { lifetime, .. })) => lifetime,
        _ => panic!("Expected fetch lifetime as first argument"),
    };
    let mut fetch_generics = ast.generics.clone();
    fetch_generics.params = fetch_generics.params.iter().cloned().skip(1).collect();
    let (fetch_impl_generics, fetch_ty_generics, fetch_where_clause) =
        fetch_generics.split_for_impl();
    let fetch_type = syn::parse_str::<syn::Ident>(&format!("{}Fetch", input_type)).unwrap();
    let crate_path = syn::parse_str::<syn::Path>("ecs").unwrap();

    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let field_tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();
            let field_names: Vec<_> = fields
                .iter()
                .map(|field| field.ident.as_ref().unwrap())
                .collect();
            let field_fetches: Vec<_> = fields
                .iter()
                .map(|field| {
                    let ty = &field.ty;
                    quote! {
                        <<#ty as #crate_path::Query>::Fetch as #crate_path::Fetch<#fetch_lifetime>>
                    }
                })
                .collect();

            let fetch_field_tys = field_tys.iter().map(|&ty| {
                struct Visitor<'a> {
                    replace: &'a syn::Lifetime,
                }
                impl syn::visit_mut::VisitMut for Visitor<'_> {
                    fn visit_lifetime_mut(&mut self, l: &mut syn::Lifetime) {
                        if l == self.replace {
                            *l = syn::Lifetime::new("'static", proc_macro2::Span::call_site());
                        }
                    }
                }

                let mut ty = ty.clone();
                syn::visit_mut::visit_type_mut(
                    &mut Visitor {
                        replace: fetch_lifetime,
                    },
                    &mut ty,
                );
                quote! {
                    <#ty as #crate_path::Query>::Fetch
                }
            });

            let expanded = quote! {
                impl #impl_generics #crate_path::Query for #input_type #ty_generics #where_clause {
                    type Fetch = #fetch_type #fetch_ty_generics;
                }

                struct #fetch_type #fetch_generics {
                    #(#field_names: #fetch_field_tys,)*
                }

                impl #fetch_impl_generics Default for #fetch_type #fetch_ty_generics #fetch_where_clause {
                    fn default() -> Self {
                        Self {
                            #(#field_names: Default::default(),)*
                        }
                    }
                }

                unsafe impl #impl_generics #crate_path::Fetch<#fetch_lifetime> for #fetch_type #fetch_ty_generics #fetch_where_clause {
                    type Output = #input_type #ty_generics;
                    type WorldBorrows = (#(#field_fetches::WorldBorrows,)*);
                    unsafe fn borrow_world(&self, world: &'a #crate_path::World) -> Option<Self::WorldBorrows> {
                        let (#(#field_names,)*) = (#(&self.#field_names,)*);
                        #(let #field_names = #field_fetches::borrow_world(#field_names, world)?;)*
                        Some((#(#field_names,)*))
                    }
                    unsafe fn get_world(&self, borrows: &Self::WorldBorrows, id: #crate_path::Id) -> Option<Self::Output> {
                        use #crate_path::util::{AsRefExt as _, ZipExt as _};
                        let fetches = (#(&self.#field_names,)*);
                        let (#(#field_names,)*) = fetches.as_ref().zip(borrows.as_ref());
                        #(
                            let (fetch, borrows) = #field_names;
                            let #field_names = #field_fetches::get_world(fetch, borrows, id)?;
                        )*
                        Some(#input_type { #(#field_names,)* })
                    }
                    type DirectBorrows = (#(<<#field_tys as #crate_path::Query>::Fetch as #crate_path::Fetch<#fetch_lifetime>>::DirectBorrows,)*);
                    unsafe fn borrow_direct(&self, entity: &'a #crate_path::Entity) -> Option<Self::DirectBorrows> {
                        let (#(#field_names,)*) = (#(&self.#field_names,)*);
                        #(let #field_names = #field_fetches::borrow_direct(#field_names, entity)?;)*
                        Some((#(#field_names,)*))
                    }
                    unsafe fn get(&self, borrows: &Self::DirectBorrows) -> Self::Output {
                        use #crate_path::util::{AsRefExt as _, ZipExt as _};
                        let fetches = (#(&self.#field_names,)*);
                        let (#(#field_names,)*) = fetches.as_ref().zip(borrows.as_ref());
                        #(
                            let (fetch, borrows) = #field_names;
                            let #field_names = #field_fetches::get(fetch, borrows);
                        )*
                        #input_type { #(#field_names,)* }
                    }
                }
            };
            expanded
        }
        _ => unimplemented!(),
    }
}
