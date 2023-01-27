use super::*;

pub struct Input {
    path: syn::LitStr,
    // ident: syn::Ident,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        // let ident = if input.peek(syn::Token!(as)) {
        //     input.parse::<syn::Token!(as)>()?;
        //     input.parse()?
        // } else {
        //     syn::Ident::new("I18N", Span::call_site())
        // };
        Ok(Self {
            path,
            // ident,
        })
    }
}

type Locale = std::collections::HashMap<String, String>;

fn parse_toml(path: impl AsRef<std::path::Path>) -> std::collections::HashMap<String, Locale> {
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut toml = String::new();
    std::io::Read::read_to_string(&mut reader, &mut toml).unwrap();
    toml::from_str(&toml).expect("Failed to parse toml")
}

pub fn process(input: Input) -> TokenStream {
    let locales = parse_toml({
        let manifest_dir: std::path::PathBuf =
            std::env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
        manifest_dir.join(input.path.value())
    });
    if locales.is_empty() {
        panic!("No locale files found");
    }
    let (first_locale, field_names): (String, std::collections::HashSet<String>) = {
        let (name, locale) = locales.iter().next().unwrap();
        (name.clone(), locale.keys().cloned().collect())
    };
    for (name, locale) in &locales {
        if field_names != locale.keys().cloned().collect() {
            if let Some(key) = locale.keys().find(|key| !field_names.contains(*key)) {
                panic!("{name:?} has {key:?} but {first_locale:?} does not");
            } else if let Some(key) = field_names.iter().find(|key| !locale.contains_key(*key)) {
                panic!("{first_locale:?} has {key:?} but {name:?} does not");
            } else {
                unreachable!()
            }
        }
    }
    let fields = field_names.iter().map(|name| {
        let name = syn::Ident::new(name, Span::call_site());
        quote! { #name: &'static str }
    });
    let locale_matches = locales.keys().map(|locale| {
        let lower = locale.to_lowercase();
        let name = syn::Ident::new(&locale.to_uppercase(), Span::call_site());
        quote! {
            #lower => &#name
        }
    });
    let locales = locales.iter().map(|(name, locale)| {
        let name = syn::Ident::new(&name.to_uppercase(), Span::call_site());
        let fields = locale.iter().map(|(key, value)| {
            let field_name = syn::Ident::new(key, Span::call_site());
            quote! { #field_name: #value }
        });
        quote! {
            pub static #name: Locale = Locale {
                #(#fields,)*
            };
        }
    });
    let locale_methods = field_names.iter().map(|name| {
        let name = syn::Ident::new(name, Span::call_site());
        quote! {
            pub fn #name(&self) -> &'static str {
                self.#name
            }
        }
    });

    quote! {
        mod i18n {
            pub struct Locale {
                #(#fields,)*
            }

            impl Locale {
                #(#locale_methods)*
            }

            pub fn get(locale: &str) -> Option<&'static Locale> {
                Some(match locale {
                    #(#locale_matches,)*
                    _ => return None,
                })
            }

            pub fn get_or_en(locale: &str) -> &'static Locale {
                get(locale).unwrap_or(&EN)
            }

            #(#locales)*
        }
    }
}
