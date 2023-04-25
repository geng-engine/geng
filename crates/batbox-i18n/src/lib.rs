//! Internationalization
//!
//! Use `batbox_i18n::gen!(mod mod_name: "path/to.toml")`
//!
//! Example toml file:
//!
//! ```toml
//! [en]
//! hello = "Hello"
//! world = "World"
//!
//! [ru]
//! hello = "Привет"
//! world = "Мир"
//! ```
//!
//! Generated code with `batbox_i18n::gen!(mod i18n: "hello_world.toml")`:
//!
//! ```ignore
//! mod i18n {
//!     struct Locale {
//!         ..
//!     }
//!
//!     fn get(locale: &str) -> Option<&'static Locale> { .. }
//!     fn get_or_en(locale: &str) -> &'static Locale { .. }
//!
//!     impl Locale {
//!         pub fn hello(&self) -> &str { .. }
//!         pub fn world(&self) -> &str { .. }
//!     }
//! }
//! ```

pub use batbox_i18n_macro::gen;

/// Detect user's locale
///
/// If detection failed, defaults to "en"
pub fn detect_locale() -> &'static str {
    static CELL: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
        #[cfg(target_arch = "wasm32")]
        let locale = web_sys::window().unwrap().navigator().language();
        #[cfg(not(target_arch = "wasm32"))]
        let locale = unsafe {
            let locale = libc::setlocale(
                libc::LC_COLLATE,
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"\0").as_ptr(),
            );
            if locale.is_null() {
                None
            } else {
                std::ffi::CStr::from_ptr(locale)
                    .to_str()
                    .ok()
                    .map(|s| s.to_owned())
            }
        };
        log::trace!("Detected locale: {:?}", locale);
        let mut locale = match locale {
            Some(locale) => locale,
            None => String::from("en"),
        };
        if locale.len() > 2 {
            locale.truncate(2);
        }
        let locale = locale.to_lowercase();
        log::trace!("Using locale: {:?}", locale);
        locale
    });
    CELL.as_str()
}
