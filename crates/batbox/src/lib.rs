#[allow(unused_imports)]
use crate as batbox;

pub mod crates {
    pub use ::anyhow;
    pub use ::async_trait;
    pub use ::bincode;
    pub use ::clap;
    pub use ::derive_more;
    pub use ::futures;
    pub use ::itertools;
    #[cfg(target_arch = "wasm32")]
    pub use ::js_sys;
    pub use ::log;
    pub use ::maplit;
    pub use ::once_cell;
    pub use ::pin_utils;
    pub use ::serde;
    pub use ::thiserror;
    #[cfg(not(target_arch = "wasm32"))]
    pub use ::threadpool;
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
pub mod result_ext;
pub mod rng;
pub mod time;
pub mod updater;
pub mod util;
