#![recursion_limit = "128"]

#[doc(hidden)]
pub use ::field_offset as __field_offset;

use batbox_color::*;
use batbox_la::*;
use batbox_range::*;
use derive_more::Deref;
use std::cell::{Cell, Ref};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

pub use ugli_raw as raw;

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
macro_rules! field_offset {
    (Self.$field_name:ident) => {
        $crate::__field_offset::offset_of!(Self => $field_name).get_byte_offset()
    }
}

#[macro_export]
macro_rules! uniforms {
    () => {
        ()
    };
    ($name:ident $([$index: literal])? : $value:expr) => {
        $crate::SingleUniform::new(stringify!($name$([$index])?), $value)
    };
    ($name:ident $([$index: literal])? : $value:expr, $($names:ident $([$indices: literal])? : $values:expr),+) => {
        ($crate::uniforms!($name$([$index])? : $value), $crate::uniforms!($($names$([$indices])? : $values),+))
    };
    ($($name:ident $([$index: literal])? : $value:expr),*,) => {
        $crate::uniforms!($($name$([$index])? : $value),*)
    }
}
