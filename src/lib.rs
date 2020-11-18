#![recursion_limit = "256"]

pub use geng_derive::*;

pub mod prelude {
    pub use crate::{draw_2d, Geng};
    pub use ::batbox::*;
    pub use ugli::{self, Ugli};
}

use crate as geng;
use crate::prelude::*;
#[allow(unused_imports)]
use log::{trace, warn};

mod asset;
mod context;
mod debug_overlay;
pub mod draw_2d;
mod font;
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
pub use context::*;
pub use debug_overlay::*;
pub use draw_2d::Draw2D;
pub use font::*;
pub use loading_screen::*;
pub use shader_lib::*;
#[cfg(feature = "audio")]
pub use sound::*;
pub use state::*;
pub use texture_atlas::*;
pub use window::*;
