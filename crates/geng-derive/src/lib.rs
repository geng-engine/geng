#![recursion_limit = "128"]
#![allow(unused_imports)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

use batbox::*;
use proc_macro2::TokenStream;

mod assets;
mod configurable;

#[proc_macro_derive(Assets, attributes(asset))]
pub fn derive_assets(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    assets::derive(input.into()).into()
}

#[proc_macro_derive(Configurable)]
pub fn derive_configurable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    configurable::derive(input.into()).into()
}
