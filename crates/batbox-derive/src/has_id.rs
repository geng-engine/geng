use darling::FromDeriveInput;

use super::*;

#[derive(FromDeriveInput)]
#[darling(supports(struct_any))]
pub struct DeriveInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), Field>,
}

#[derive(FromField)]
#[darling(attributes(has_id))]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    id: bool,
}

impl DeriveInput {
    pub fn derive(self) -> TokenStream {
        let Self {
            ident,
            generics,
            data,
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let data = data.take_struct().unwrap();
        fn find_field_with_id_attr(fields: &[Field]) -> Option<(usize, &Field)> {
            let mut result = None;
            for (index, field) in fields.iter().enumerate() {
                if field.id {
                    assert!(result.is_none(), "Only one field must have id attr");
                    result = Some((index, field));
                }
            }
            result
        }
        fn find_field_with_id_name(fields: &[Field]) -> Option<(usize, &Field)> {
            fields
                .iter()
                .enumerate()
                .find(|(_, field)| field.ident.as_ref().map_or(false, |ident| ident == "id"))
        }
        let (id_field_index, id_field) = find_field_with_id_attr(&data.fields)
            .or_else(|| find_field_with_id_name(&data.fields))
            .expect("Expected field with #[id] attr or named `id`");
        let id_field_ty = &id_field.ty;
        let id_field_index = syn::Index::from(id_field_index);
        let id_field_ref = match &id_field.ident {
            Some(ident) => quote! { #ident },
            None => quote! { #id_field_index },
        };
        quote! {
            impl #impl_generics batbox::collection::HasId for #ident #ty_generics #where_clause {
                type Id = #id_field_ty;
                fn id(&self) -> &Self::Id {
                    &self.#id_field_ref
                }
            }
        }
    }
}
