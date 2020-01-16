#![recursion_limit = "128"]
#![allow(unused_imports)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

use batbox::*;
use itertools::izip;
use proc_macro2::TokenStream;

mod assets;

#[proc_macro_derive(Assets, attributes(asset))]
pub fn derive_assets(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    assets::derive(input.into()).into()
}
