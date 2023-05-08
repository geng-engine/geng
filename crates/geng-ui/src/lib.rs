use batbox_cmp::*;
use batbox_color::*;
use batbox_la::*;
use batbox_tuple_macros::*;
use derive_more::{Deref, DerefMut};
use geng_camera::PixelPerfectCamera;
use geng_draw2d as draw2d;
use geng_font::{Font, TextAlign};
use geng_window::{Event, Touch};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::ops::RangeInclusive;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use ugli::Ugli;

mod config;
mod controller;
mod layout_widgets;
mod theme;
pub mod widget;
mod widgets;

pub use config::*;
pub use controller::*;
pub use layout_widgets::*;
pub use theme::*;
pub use widget::*;
pub use widgets::*;

fn traverse_mut(
    widget: &mut dyn Widget,
    on_enter: &mut dyn FnMut(&mut dyn Widget),
    on_leave: &mut dyn FnMut(&mut dyn Widget),
) {
    on_enter(widget);
    widget.walk_children_mut(&mut |widget| traverse_mut(widget, on_enter, on_leave));
    on_leave(widget);
}
