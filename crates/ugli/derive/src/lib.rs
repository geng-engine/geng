#![recursion_limit = "128"]
#![allow(unused_imports)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

use batbox::*;
use proc_macro2::TokenStream;

mod uniforms;

#[proc_macro_derive(Uniforms)]
pub fn derive_uniforms(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    uniforms::derive(input.into()).into()
}

mod vertex;

#[proc_macro_derive(Vertex)]
pub fn derive_vertex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    vertex::derive(input.into()).into()
}

fn simple_derive(
    input: TokenStream,
    typ: syn::Path,
    expand: fn(&syn::DeriveInput) -> TokenStream,
) -> TokenStream {
    let s = input.to_string();
    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();
    let input_type = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let impl_body = expand(&ast);
    quote! {
        impl#impl_generics #typ for #input_type#ty_generics #where_clause {
            #impl_body
        }
    }
}
