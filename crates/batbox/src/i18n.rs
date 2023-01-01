//! Internationalization
//!
//! Use `i18n!("path/to.toml")` to generate i18n mod
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
//! Generated code:
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
use super::*;

pub mod prelude {
    //! Items intended to always be available. Reexported from [crate::prelude]

    #[doc(no_inline)]
    pub use crate::file;
    // pub use batbox_derive::i18n;
}

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
        trace!("Detected locale: {:?}", locale);
        let mut locale = match locale {
            Some(locale) => locale,
            None => String::from("en"),
        };
        if locale.len() > 2 {
            locale.truncate(2);
        }
        let locale = locale.to_lowercase();
        trace!("Using locale: {:?}", locale);
        locale
    });
    CELL.as_str()
}
