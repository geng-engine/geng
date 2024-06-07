use proc_macro2::Span;

use super::*;

#[derive(FromDeriveInput)]
#[darling(supports(struct_any))]
pub struct DeriveInput {
    vis: syn::Visibility,
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
            vis,
            ident,
            generics,
            data,
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let data = data.take_struct().unwrap();
        let field_names: Vec<_> = data
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                field
                    .ident
                    .as_ref()
                    .map(|ident| quote! { #ident })
                    .unwrap_or_else(|| {
                        let index = syn::Index::from(index);
                        quote! { #index }
                    })
            })
            .collect();
        let field_tys = data.fields.iter().map(|field| &field.ty);
        let field_count = data.fields.len();
        let cache_key_type_name =
            syn::Ident::new(&format!("{ident}CacheKeyType"), Span::call_site());
        quote! {
            #[allow(non_camel_case_types)]
            #vis struct #cache_key_type_name <#(#field_names: 'static,)*> {
                #(
                    #field_names: std::marker::PhantomData<#field_names>,
                )*
            }
            #[allow(non_camel_case_types)]
            impl #impl_generics ugli::Uniforms for #ident #ty_generics #where_clause {
                type ProgramInfoCacheKey = #cache_key_type_name <#(<#field_tys as ugli::Uniform>::LifetimeErased,)*>;
                type ProgramInfo = [Option<ugli::UniformInfo>; #field_count];
                fn get_program_info(program: &ugli::Program) -> Self::ProgramInfo {
                    [#(
                        program.uniform_info(stringify!(#field_names)),
                    )*]
                }
                fn apply_uniforms(&self, program: &ugli::Program, info: &Self::ProgramInfo) {
                    let [#(#field_names,)*] = info;
                    #(
                        if let Some(info) = #field_names {
                            ugli::Uniform::apply(&self.#field_names, program, info);
                        }
                    )*
                }
            }
        }
    }
}
