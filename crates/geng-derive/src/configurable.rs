use super::*;

pub fn derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();
    let input_type = &ast.ident;
    // let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let field_tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();
            let field_tys = &field_tys;
            let field_names: Vec<_> = fields
                .iter()
                .map(|field| field.ident.as_ref().unwrap())
                .collect();
            let field_names = &field_names;
            let field_names_copy = field_names.clone();
            let config_ident = syn::Ident::new(
                &format!("{}Config", input_type),
                proc_macro2::Span::call_site(),
            );

            quote! {
                pub struct #config_ident {
                    theme: Rc<geng::ui::Theme>,
                    #(#field_names: <#field_tys as geng::ui::Configurable>::Config,)*
                }

                impl geng::ui::Config<#input_type> for #config_ident {
                    fn get(&self) -> #input_type {
                        #input_type {
                            #(#field_names: self.#field_names_copy.get(),)*
                        }
                    }
                    fn ui<'a>(&'a mut self) -> Box<dyn geng::ui::Widget + 'a> {
                        use geng::ui::*;
                        Box::new(geng::ui::column! [
                            #(geng::ui::row! [
                                geng::ui::text(stringify!(#field_names), &self.theme.font, 16.0, Color::GRAY).align(vec2(0.0, 0.5)),
                                self.#field_names_copy.ui().flex_align(vec2(Some(1.0), None), vec2(1.0, 0.5)),
                            ],)*
                        ])
                    }
                }

                impl geng::ui::Configurable for #input_type {
                    type Config = #config_ident;
                    fn config(theme: &Rc<geng::ui::Theme>, value: Self) -> Self::Config {
                        Self::Config {
                            theme: theme.clone(),
                            #(#field_names: <#field_tys as geng::ui::Configurable>::config(theme, value.#field_names_copy),)*
                        }
                    }
                }
            }
        }
        _ => panic!("geng::ui::Configurable can only be derived by structs"),
    }
}
