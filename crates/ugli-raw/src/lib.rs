#[cfg(not(target_arch = "wasm32"))]
#[path = "gl/mod.rs"]
mod implementation;

#[cfg(target_arch = "wasm32")]
#[path = "webgl/mod.rs"]
mod implementation;

pub use implementation::*;
