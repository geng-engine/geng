use super::*;

pub fn derive(input: TokenStream) -> TokenStream {
    simple_derive(
        input,
        false,
        syn::parse_str("ugli::Vertex").unwrap(),
        expand,
    )
}

pub fn expand(input: &syn::DeriveInput) -> TokenStream {
    match input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
            let field_names_copy = fields.iter().map(|field| field.ident.as_ref().unwrap());
            quote! {
                unsafe fn walk_attributes<C>(sample: *const Self, mut visitor: C)
                    where C: ugli::VertexAttributeVisitor {
                    #(visitor.visit(
                        stringify!(#field_names_copy),
                        unsafe {
                            std::ptr::addr_of!((*sample).#field_names)
                        },
                    ));*
                }
            }
        }
        _ => panic!("ugli::Vertex can only be derived by structs"),
    }
}