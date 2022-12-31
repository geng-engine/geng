#![recursion_limit = "128"]
#![allow(unused_imports)]

extern crate proc_macro;

use darling::{FromDeriveInput, FromField};
use proc_macro2::{Span, TokenStream};
use quote::quote;

mod diff;
mod has_id;
mod i18n;

#[proc_macro_derive(Diff, attributes(diff))]
pub fn derive_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    diff::derive(input.into()).into()
}

#[proc_macro_derive(HasId, attributes(has_id))]
pub fn derive_has_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    match has_id::DeriveInput::from_derive_input(&input) {
        Ok(input) => input.derive().into(),
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro]
pub fn i18n(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    i18n::process(syn::parse_macro_input!(input)).into()
}
