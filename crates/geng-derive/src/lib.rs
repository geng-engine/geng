#![recursion_limit = "128"]
#![allow(unused_imports)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

use batbox::prelude::*;
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro2::TokenStream;

mod assets;
mod configurable;

#[proc_macro_derive(Assets, attributes(asset))]
pub fn derive_assets(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    match assets::DeriveInput::from_derive_input(&input) {
        Ok(input) => input.derive().into(),
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro_derive(Configurable)]
pub fn derive_configurable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    configurable::derive(input.into()).into()
}
