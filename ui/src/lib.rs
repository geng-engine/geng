use geng_core as geng;
use geng_core::*;
use prelude::*;

pub use geng_ui_derive::*;

mod config;
mod layout_widgets;
mod theme;
pub mod widget;
mod widgets;

pub use config::*;
pub use layout_widgets::*;
pub use theme::*;
pub use widget::*;
pub use widgets::*;

use geng_core::Event as RawEvent;

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
    captured_id: Option<ID>,
    last_touch_pos: Option<Vec2<f64>>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            captured_id: None,
            last_touch_pos: None,
        }
    }
}

fn traverse_mut<F: FnMut(&mut dyn Widget), G: FnMut(&mut dyn Widget)>(
    widget: &mut dyn Widget,
    on_enter: &mut F,
    on_leave: &mut G,
) {
    on_enter(widget);
    widget.walk_children_mut(Box::new(|widget| traverse_mut(widget, on_enter, on_leave)));
    on_leave(widget);
}

impl Controller {
    pub fn update<T: Widget>(&mut self, mut root: T, delta_time: f64) {
        traverse_mut(
            &mut root,
            &mut |widget| widget.update(delta_time),
            &mut |_| {},
        );
    }
    pub fn draw<T: Widget>(&mut self, mut root: T, framebuffer: &mut ugli::Framebuffer) {
        traverse_mut(&mut root, &mut |_| {}, &mut |widget| {
            widget.calc_constraints();
        });
        root.calc_constraints();
        root.core_mut().position =
            AABB::from_corners(vec2(0.0, 0.0), framebuffer.size().map(|x| x as f64));
        traverse_mut(
            &mut root,
            &mut |widget| {
                widget.layout_children();
            },
            &mut |_| {},
        );
        traverse_mut(
            &mut root,
            &mut |widget| {
                widget.draw(framebuffer);
            },
            &mut |_| {},
        );
    }
    pub fn handle_event<T: Widget>(&mut self, mut root: T, event: RawEvent) -> bool {
        traverse_mut(
            &mut root,
            &mut |widget| {
                if let Some(id) = self.captured_id {
                    if id != widget.core().id {
                        return;
                    }
                }
                match event {
                    RawEvent::MouseMove { position } => {
                        widget.core_mut().hovered = widget.core().position.contains(position);
                        widget.handle_event(&Event::MouseMove { position });
                    }
                    RawEvent::MouseDown { button, position } => {
                        if widget.core().position.contains(position) {
                            widget.core_mut().captured = true;
                            self.captured_id = Some(widget.core().id);
                            widget.handle_event(&Event::MouseDown { button, position });
                        } else if widget.core().captured {
                            widget.handle_event(&Event::MouseDown { button, position });
                        }
                    }
                    RawEvent::MouseUp { button, position } => {
                        let was_captured = widget.core().captured;
                        widget.core_mut().captured = false;
                        if was_captured {
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
                            self.captured_id = Some(widget.core().id);
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
                }
            },
            &mut |_| {},
        );
        match event {
            RawEvent::MouseDown { .. } | RawEvent::TouchStart { .. } => {
                if self.captured_id.is_none() {
                    self.captured_id = Some(ID(IDImpl::Void));
                }
            }
            RawEvent::MouseUp { .. } | RawEvent::TouchEnd => {
                self.captured_id = None;
            }
            _ => {}
        }
        match self.captured_id {
            Some(ID(IDImpl::Void)) | None => false,
            _ => true,
        }
    }
}
