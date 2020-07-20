#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

#[cfg(not(target_arch = "wasm32"))]
#[path = "gl/mod.rs"]
mod implementation;

#[cfg(target_arch = "wasm32")]
#[path = "webgl/mod.rs"]
mod implementation;

pub use implementation::*;
