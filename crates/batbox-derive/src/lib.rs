#![recursion_limit = "128"]
#![allow(unused_imports)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;

mod diff;
mod has_id;

#[proc_macro_derive(Diff, attributes(diff))]
pub fn derive_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    diff::derive(input.into()).into()
}

#[proc_macro_derive(HasId, attributes(id))]
pub fn derive_has_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    has_id::derive(input.into()).into()
}
