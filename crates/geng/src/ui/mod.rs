use super::*;

pub use geng_ui_macros::{column, row, stack};

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
    widget.walk_children_mut(Box::new(|widget| traverse_mut(widget, on_enter, on_leave)));
    on_leave(widget);
}
