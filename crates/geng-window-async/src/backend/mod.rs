use super::*;

#[cfg(not(target_arch = "wasm32"))]
mod glutin_winit;
#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(not(target_arch = "wasm32"))]
pub use self::glutin_winit::*;
#[cfg(target_arch = "wasm32")]
pub use self::web::*;
