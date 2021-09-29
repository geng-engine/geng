use super::*;

pub fn derive(input: TokenStream) -> TokenStream {
    simple_derive(
        input,
        true,
        syn::parse_str("ugli::Uniforms").unwrap(),
        expand,
    )
}

fn expand(input: &syn::DeriveInput) -> TokenStream {
    match input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
            let field_names_copy = fields.iter().map(|field| field.ident.as_ref().unwrap());
            quote! {
                fn walk_uniforms<C>(&self, visitor: &mut C) where C: ugli::UniformVisitor {
                    #(visitor.visit(stringify!(#field_names_copy), &self.#field_names));*
                }
            }
        }
        _ => panic!("ugli::Uniforms can only be derived by structs"),
    }
}
