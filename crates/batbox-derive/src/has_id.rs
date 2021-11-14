use super::*;

pub fn derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();
    let input_type = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let id_field = match fields.iter().find(|field| {
                for attr in &field.attrs {
                    if let Ok(syn::Meta::Path(path)) = attr.parse_meta() {
                        if path.is_ident("id") {
                            return true;
                        }
                    }
                }
                false
            }) {
                Some(field) => field,
                None => fields
                    .iter()
                    .find(|field| field.ident.as_ref().unwrap() == "id")
                    .expect("Expected a field with #[id] attribute or with name 'id'"),
            };
            let id_ty = &id_field.ty;
            let id_field_name = id_field.ident.as_ref().unwrap();

            let expanded = quote! {
                impl #impl_generics batbox::HasId for #input_type #ty_generics #where_clause {
                    type Id = #id_ty;
                    fn id(&self) -> &Self::Id {
                        &self.#id_field_name
                    }
                }
            };
            expanded
        }
        _ => panic!("HasId can only be derived by structs"),
    }
}
