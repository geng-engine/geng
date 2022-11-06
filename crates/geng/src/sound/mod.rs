use super::*;

#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod platform_impl;
#[cfg(not(target_arch = "wasm32"))]
#[path = "native.rs"]
mod platform_impl;

pub use platform_impl::*;
