#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
#[macro_use]
extern crate stdweb;

#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
#[path = "gl/mod.rs"]
mod implementation;

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
#[path = "webgl/mod.rs"]
mod implementation;

pub use implementation::*;
