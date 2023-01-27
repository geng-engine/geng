//! Working with colors in various formats
use super::*;

pub mod prelude {
    //! Items intended to always be available. Reexported from [crate::prelude]

    #[doc(no_inline)]
    pub use super::{ColorComponent, Hsva, Rgba};
}

mod component;
mod consts;
mod hsv;
mod rgba;

pub use component::*;
pub use consts::*;
pub use hsv::*;
pub use rgba::*;
