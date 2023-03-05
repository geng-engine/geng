#![recursion_limit = "256"]
#![allow(clippy::needless_doctest_main)]
//!
//! `geng` (Game ENGine) is an engine for Rust Programming Language.
//!
//! # Quick start
//! More examples are available [here](https://github.com/kuviman/geng/tree/main/crates/geng/examples).
//!
//! ```no_run
//! use geng::prelude::*;
//!
//! struct State;
//!
//! impl geng::State for State {
//!     fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
//!         ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
//!     }
//! }
//!
//! fn main() {
//!     logger::init().unwrap();
//!     geng::setup_panic_handler();
//!     let geng = Geng::new("Application Name");
//!     let state = State;
//!     geng.run(state);
//! }
//! ```
//!

pub use geng_derive::*;

pub mod prelude {
    pub use crate::{draw_2d, Geng};
    pub use crate::{Camera2dExt as _, Camera3dExt as _};
    pub use ::batbox;
    pub use ::batbox::prelude::*;
    pub use ugli::{self, Ugli};
}

use crate as geng;
use crate::prelude::*;
#[allow(unused_imports)]
use log::{trace, warn};

mod asset;
mod camera;
mod cli_args;
mod context;
mod debug_overlay;
pub mod draw_2d;
pub mod font;
mod loading_screen;
pub mod net;
pub mod obj;
mod shader_lib;
#[cfg(feature = "audio")]
mod sound;
mod state;
mod texture_atlas;
pub mod ui;
mod window;

pub use asset::*;
pub use camera::*;
pub use cli_args::*;
pub use context::*;
pub use debug_overlay::*;
pub use draw_2d::Draw2d;
pub use font::*;
pub use loading_screen::*;
pub use shader_lib::*;
#[cfg(feature = "audio")]
pub use sound::*;
pub use state::*;
pub use texture_atlas::*;
pub use window::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn setup_panic_handler() {
    // TODO: do something useful here too?
}

#[cfg(target_arch = "wasm32")]
pub fn setup_panic_handler() {
    #[wasm_bindgen(inline_js = r#"
    export function show_error(text) {
        document.getElementById("geng-progress-screen").style.display = "none";
        document.getElementById("geng-canvas").style.display = "none";
        document.getElementById("error-message").textContent = text;
        document.getElementById("geng-error-screen").style.display = "block";
    }
    "#)]
    extern "C" {
        fn show_error(s: &str);
    }
    fn panic_hook(info: &std::panic::PanicInfo) {
        console_error_panic_hook::hook(info);
        static ALREADY_PANICKED: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);
        if ALREADY_PANICKED.swap(true, std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        let error: String = if let Some(error) = info.payload().downcast_ref::<String>() {
            error.clone()
        } else if let Some(error) = info.payload().downcast_ref::<&str>() {
            error.to_string()
        } else {
            String::from("Something went wrong")
        };
        show_error(&error);
    }
    std::panic::set_hook(Box::new(panic_hook));
}
