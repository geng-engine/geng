use super::*;

pub use geng_ui_macros::{column, row, stack};

mod config;
mod container;
mod layout_widgets;
mod theme;
pub mod widget;
mod widgets;

pub use config::*;
pub use container::*;
pub use layout_widgets::*;
pub use theme::*;
pub use widget::*;
pub use widgets::*;

use crate::Event as RawEvent;

#[derive(Debug)]
pub enum Event {
    MouseMove {
        position: Vec2<f64>,
    },
    MouseDown {
        button: MouseButton,
        position: Vec2<f64>,
    },
    MouseUp {
        button: MouseButton,
        position: Vec2<f64>,
    },
    Click {
        button: MouseButton,
    },
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum IDImpl {
    Regular(usize),
    Void,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct ID(IDImpl);

impl ID {
    fn new() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        ID(IDImpl::Regular(
            NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        ))
    }
    fn void() -> Self {
        ID(IDImpl::Void)
    }
}

pub trait State {
    fn ui<'a>(&'a mut self) -> Box<dyn Widget + 'a>;
}

pub struct Controller {
    size: Vec2<f64>,
    last_touch_pos: Option<Vec2<f64>>,
}

impl Controller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            size: vec2(1.0, 1.0),
            last_touch_pos: None,
        }
    }
}

fn traverse_mut(
    widget: &mut dyn Widget,
    on_enter: &mut dyn FnMut(&mut dyn Widget),
    on_leave: &mut dyn FnMut(&mut dyn Widget),
) {
    on_enter(widget);
    widget.walk_children_mut(Box::new(|widget| traverse_mut(widget, on_enter, on_leave)));
    on_leave(widget);
}

impl Controller {
    pub fn update(&mut self, root: &mut dyn Widget, delta_time: f64) {
        self.layout(root);
        traverse_mut(root, &mut |widget| widget.update(delta_time), &mut |_| {});
    }
    fn layout(&mut self, root: &mut dyn Widget) {
        traverse_mut(root, &mut |_| {}, &mut |widget| {
            widget.calc_constraints();
        });
        root.calc_constraints();
        root.core_mut().position = AABB::ZERO.extend_positive(self.size);
        traverse_mut(
            root,
            &mut |widget| {
                widget.layout_children();
            },
            &mut |_| {},
        );
    }
    pub fn draw(&mut self, root: &mut dyn Widget, framebuffer: &mut ugli::Framebuffer) {
        self.size = framebuffer.size().map(|x| x as f64);
        self.layout(root);
        traverse_mut(
            root,
            &mut |widget| {
                widget.draw(framebuffer);
            },
            &mut |_| {},
        );
    }
    pub fn handle_event(&mut self, root: &mut dyn Widget, event: RawEvent) -> bool {
        self.layout(root);
        let mut captured = false;
        traverse_mut(
            root,
            &mut |widget| match event {
                RawEvent::MouseMove { position, .. } => {
                    widget.core_mut().hovered = widget.core().position.contains(position);
                    widget.handle_event(&Event::MouseMove { position });
                }
                RawEvent::MouseDown { button, position } => {
                    if widget.core().position.contains(position) {
                        widget.core_mut().captured = true;
                        widget.handle_event(&Event::MouseDown { button, position });
                    } else if widget.core().captured {
                        widget.handle_event(&Event::MouseDown { button, position });
                    }
                }
                RawEvent::MouseUp { button, position } => {
                    let was_captured = widget.core().captured;
                    widget.core_mut().captured = false;
                    if was_captured || widget.core().position.contains(position) {
                        widget.handle_event(&Event::MouseUp { button, position });
                    }
                    if was_captured && widget.core().position.contains(position) {
                        widget.handle_event(&Event::Click { button });
                    }
                }
                RawEvent::TouchStart { ref touches } if touches.len() == 1 => {
                    let position = touches[0].position;
                    self.last_touch_pos = Some(position);
                    if widget.core().position.contains(position) {
                        widget.core_mut().captured = true;
                        widget.handle_event(&Event::MouseDown {
                            button: MouseButton::Left,
                            position,
                        });
                    } else if widget.core().captured {
                        widget.handle_event(&Event::MouseDown {
                            button: MouseButton::Left,
                            position,
                        });
                    }
                }
                RawEvent::TouchEnd => {
                    if let Some(position) = self.last_touch_pos {
                        let was_captured = widget.core().captured;
                        widget.core_mut().captured = false;
                        if was_captured {
                            widget.handle_event(&Event::MouseUp {
                                button: MouseButton::Left,
                                position,
                            });
                        }
                        if was_captured && widget.core().position.contains(position) {
                            widget.handle_event(&Event::Click {
                                button: MouseButton::Left,
                            });
                        }
                    }
                }
                RawEvent::TouchMove { ref touches } if touches.len() == 1 => {
                    let position = touches[0].position;
                    self.last_touch_pos = Some(position);
                    widget.core_mut().hovered = widget.core().position.contains(position);
                    widget.handle_event(&Event::MouseMove { position });
                }
                _ => {}
            },
            &mut |widget| {
                if widget.core().id != ID::void() && widget.core().captured {
                    captured = true;
                }
            },
        );
        captured
    }
}
