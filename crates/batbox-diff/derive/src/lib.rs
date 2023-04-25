#![recursion_limit = "128"]
#![allow(unused_imports)]

extern crate proc_macro;

use darling::{FromDeriveInput, FromField};
use proc_macro2::{Span, TokenStream};
use quote::quote;

#[proc_macro_derive(Diff, attributes(diff))]
pub fn derive_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_impl(input.into()).into()
}

// TODO: simplify by actually using darling
enum DiffMode {
    Diff,
    Clone,
    Eq,
}

fn derive_impl(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();
    let input_type = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let generics = &ast.generics;
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let field_tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();
            let field_tys = &field_tys;
            let field_names: Vec<_> = fields
                .iter()
                .map(|field| field.ident.as_ref().unwrap())
                .collect();
            let field_names = &field_names;
            let delta_type = syn::Ident::new(
                &format!("{input_type}Delta"),
                proc_macro2::Span::call_site(),
            );

            let field_diff_modes: Vec<DiffMode> = fields
                .iter()
                .map(|field| {
                    let mut diff_type = DiffMode::Diff;
                    for attr in &field.attrs {
                        if let Ok(syn::Meta::NameValue(syn::MetaNameValue {
                            path: ref meta_path,
                            lit: syn::Lit::Str(ref s),
                            ..
                        })) = attr.parse_meta()
                        {
                            if meta_path.is_ident("diff") {
                                diff_type = match s.value().as_str() {
                                    "eq" => DiffMode::Eq,
                                    "diff" => DiffMode::Diff,
                                    "clone" => DiffMode::Clone,
                                    _ => panic!("Unexpected diff type"),
                                }
                            }
                        }
                    }
                    diff_type
                })
                .collect();

            let field_diff_types =
                field_tys
                    .iter()
                    .zip(field_diff_modes.iter())
                    .map(|(field_ty, field_diff_mode)| match field_diff_mode {
                        DiffMode::Diff => quote! {
                            <#field_ty as Diff>::Delta
                        },
                        DiffMode::Clone => quote! {
                            #field_ty
                        },
                        DiffMode::Eq => quote! {
                            Option<#field_ty>
                        },
                    });

            let field_diffs = field_names.iter().zip(field_diff_modes.iter()).map(
                |(field_name, field_diff_mode)| match field_diff_mode {
                    DiffMode::Diff => quote! {
                        Diff::diff(&self.#field_name, &to.#field_name)
                    },
                    DiffMode::Clone => quote! {
                        to.#field_name.clone()
                    },
                    DiffMode::Eq => quote! {
                        if self.#field_name == to.#field_name {
                            None
                        } else {
                            Some(to.#field_name.clone())
                        }
                    },
                },
            );

            let field_updates = field_names.iter().zip(field_diff_modes.iter()).map(
                |(field_name, field_diff_mode)| match field_diff_mode {
                    DiffMode::Diff => quote! {
                        Diff::update(&mut self.#field_name, &delta.#field_name);
                    },
                    DiffMode::Clone => quote! {
                        self.#field_name = delta.#field_name.clone();
                    },
                    DiffMode::Eq => quote! {
                        if let Some(value) = &delta.#field_name {
                            self.#field_name = value.clone();
                        }
                    },
                },
            );

            let expanded = quote! {
                #[derive(Debug, Serialize, Deserialize, Clone)]
                pub struct #delta_type #generics {
                    #(#field_names: #field_diff_types,)*
                }

                impl #impl_generics Diff for #input_type #ty_generics #where_clause {
                    type Delta = #delta_type;
                    fn diff(&self, to: &Self) -> Self::Delta {
                        #delta_type {
                            #(#field_names: #field_diffs,)*
                        }
                    }
                    fn update(&mut self, delta: &Self::Delta) {
                        #(#field_updates)*
                    }
                }
            };
            expanded
        }
        _ => panic!("Diff can only be derived by structs"),
    }
}
