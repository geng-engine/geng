use super::*;

#[derive(FromDeriveInput)]
#[darling(supports(struct_any))]
pub struct DeriveInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), Field>,
}

#[derive(FromField)]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
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
        let field_tys = data.fields.iter().map(|field| &field.ty);
        let field_names = data.fields.iter().enumerate().map(|(index, field)| {
            field
                .ident
                .as_ref()
                .map(|ident| quote! { #ident })
                .unwrap_or_else(|| {
                    let index = syn::Index::from(index);
                    quote! { #index }
                })
        });
        quote! {
            unsafe impl #impl_generics ugli::Vertex for #ident #ty_generics #where_clause {
                fn walk_attributes(mut visitor: impl ugli::VertexAttributeVisitor) {
                    #(visitor.visit::<#field_tys>(
                        stringify!(#field_names),
                        ugli::field_offset!(Self.#field_names),
                    ));*
                }
            }
        }
    }
}
