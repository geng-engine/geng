#![recursion_limit = "128"]

use batbox::prelude::*;
pub use ugli_derive::*;

mod context;
mod draw;
mod error;
mod framebuffer;
mod program;
mod renderbuffer;
mod shader;
mod texture;
mod uniform;
mod vertex;

pub use context::*;
pub use draw::*;
pub use error::*;
pub use framebuffer::*;
pub use program::*;
pub use renderbuffer::*;
pub use shader::*;
pub use texture::*;
pub use uniform::*;
pub use vertex::*;

pub use raw;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct DepthComponent(raw::Float);

fn gl_bool(b: bool) -> raw::Bool {
    if b {
        raw::TRUE
    } else {
        raw::FALSE
    }
}

#[macro_export]
macro_rules! uniforms {
    () => {
        ()
    };
    ($name:ident : $value:expr) => {
        $crate::SingleUniform::new(stringify!($name), $value)
    };
    ($name:ident : $value:expr, $($names:ident : $values:expr),+) => {
        ($crate::uniforms!($name : $value), $crate::uniforms!($($names : $values),+))
    };
    ($($name:ident : $value:expr),*,) => {
        $crate::uniforms!($($name : $value),*)
    }
}
