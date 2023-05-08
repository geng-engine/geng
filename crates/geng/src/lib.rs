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
//!     logger::init();
//!     geng::setup_panic_handler();
//!     let geng = Geng::new("Application Name");
//!     let state = State;
//!     geng.run(state);
//! }
//! ```
//!

pub mod prelude {
    pub use crate::{draw2d, Geng, Hot};
    pub use crate::{AbstractCamera2d, AbstractCamera3d, Camera2d};
    pub use ::batbox;
    pub use ::batbox::prelude::*;
    pub use gilrs::{self, Gilrs};
    pub use ugli::{self, Ugli};
}

use crate::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod cli_args;
mod context;
mod loading_screen;

pub use geng_asset as asset;
#[cfg(feature = "audio")]
pub use geng_audio::{self as audio, *};
pub use geng_camera::{
    self as camera, AbstractCamera2d, AbstractCamera3d, Camera2d, PixelPerfectCamera,
};
pub use geng_draw2d::{self as draw2d, Draw2d};
pub use geng_font::{self as font, Font, TextAlign};
pub use geng_net as net;
pub use geng_shader as shader;
pub use geng_state::{self as state, State};
pub use geng_texture_atlas::{self as texture_atlas, TextureAtlas};
pub use geng_ui as ui;
pub use geng_window::{self as window, CursorType, Event, Key, MouseButton, Touch, Window};

pub use asset::*;
pub use cli_args::*;
pub use context::*;
pub use loading_screen::*;

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

impl Geng {
    pub(crate) fn set_loading_progress_title(&self, title: &str) {
        log::trace!("Set loading progress title to {title:?}");
        // TODO: native
        #[cfg(target_arch = "wasm32")]
        {
            #[wasm_bindgen(inline_js = r#"
            export function set_progress_title(title) {
                window.gengUpdateProgressTitle(title);
            }
            "#)]
            extern "C" {
                fn set_progress_title(title: &str);
            }
            set_progress_title(title);
        }
    }

    pub(crate) fn set_loading_progress(&self, progress: f64, total: Option<f64>) {
        log::trace!("Loading progress {progress:?}/{total:?}");
        // TODO: native
        #[cfg(target_arch = "wasm32")]
        {
            #[wasm_bindgen(inline_js = r#"
            export function set_progress(progress, total) {
                window.gengUpdateProgress(progress, total);
            }
            "#)]
            extern "C" {
                fn set_progress(progress: f64, total: Option<f64>);
            }
            set_progress(progress, total);
        }
    }

    pub(crate) fn finish_loading(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            #[wasm_bindgen(inline_js = r#"
            export function finish_loading() {
                document.getElementById("geng-progress-screen").style.display = "none";
                document.getElementById("geng-canvas").style.display = "block";
            }
            "#)]
            extern "C" {
                fn finish_loading();
            }
            finish_loading();
        }
    }
}
