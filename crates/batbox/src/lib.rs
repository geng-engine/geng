//! Battery box, a library containing common stuff
//!
//! Check out [prelude] which is supposed to be used like `use batbox::prelude::*`
//! A lot of reexports of std and other [crates].

#![warn(missing_docs)]

#[allow(unused_imports)]
use crate as batbox;

pub mod crates {
    //! External crates

    pub use ::anyhow;
    pub use ::async_trait;
    pub use ::bincode;
    pub use ::clap;
    pub use ::derivative;
    pub use ::derive_more;
    pub use ::dyn_clone;
    pub use ::env_logger;
    pub use ::futures;
    pub use ::itertools;
    #[cfg(target_arch = "wasm32")]
    pub use ::js_sys;
    pub use ::log;
    pub use ::maplit;
    pub use ::once_cell;
    pub use ::pin_utils;
    pub use ::serde;
    pub use ::serde_json;
    pub use ::thiserror;
    #[cfg(not(target_arch = "wasm32"))]
    pub use ::threadpool;
    pub use ::toml;
    #[cfg(target_arch = "wasm32")]
    pub use ::wasm_bindgen;
    #[cfg(target_arch = "wasm32")]
    pub use ::wasm_bindgen_futures;
    #[cfg(target_arch = "wasm32")]
    pub use ::web_sys;
}

pub mod prelude;
#[doc(no_inline)]
pub use crates::*;
use prelude::*;

#[doc(no_inline)]
pub use batbox_derive::*;
#[doc(no_inline)]
pub use batbox_macros::*;

pub mod approx;
pub mod cmp;
pub mod collection;
pub mod color;
pub mod diff;
pub mod file;
pub mod geom;
pub mod i18n;
pub mod logger;
pub mod num;
pub mod preferences;
pub mod program_args;
pub mod range;
pub mod rng;
pub mod time;
pub mod util;
