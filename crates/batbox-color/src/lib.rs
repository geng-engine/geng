//! Working with colors in various formats
#![warn(missing_docs)]

use batbox_approx::Approx;
use batbox_la::*;

mod component;
mod consts;
mod hsl;
mod hsv;
mod rgba;

pub use component::*;
pub use consts::*;
pub use hsl::*;
pub use hsv::*;
pub use rgba::*;
