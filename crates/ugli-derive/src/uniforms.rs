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
            impl #impl_generics ugli::Uniforms for #ident #ty_generics #where_clause {
                fn walk_uniforms<C>(&self, visitor: &mut C) where C: ugli::UniformVisitor {
                    #(visitor.visit(stringify!(#field_names), &self.#field_names));*
                }
            }
        }
    }
}
