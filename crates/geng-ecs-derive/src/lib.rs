extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;

mod query;

#[proc_macro_derive(Query)]
pub fn derive_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    query::derive(input.into()).into()
}
