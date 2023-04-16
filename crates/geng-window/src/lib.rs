use batbox_la::*;
use serde::{Deserialize, Serialize};
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::Rc;
use ugli::Ugli;

mod platform;

mod cursor;
mod events;

pub use cursor::*;
pub use events::*;

pub struct Options {
    pub fullscreen: bool,
    pub vsync: bool,
    pub title: String,
    pub antialias: bool,
    pub transparency: bool,
    pub size: Option<vec2<usize>>,
}

pub struct Window {
    platform: platform::Context,
    #[allow(clippy::type_complexity)]
    event_handler: Rc<RefCell<Option<Box<dyn FnMut(Event)>>>>,
    pressed_keys: Rc<RefCell<HashSet<Key>>>,
    pressed_buttons: Rc<RefCell<HashSet<MouseButton>>>,
}

impl Window {
    pub fn new(options: &Options) -> Self {
        let window = Self {
            platform: platform::Context::new(options),
            event_handler: Rc::new(RefCell::new(None)),
            pressed_keys: Rc::new(RefCell::new(HashSet::new())),
            pressed_buttons: Rc::new(RefCell::new(HashSet::new())),
        };
        if options.fullscreen {
            window.set_fullscreen(true);
        }
        window
    }

    /// TODO internal?
    pub fn send_event(&self, event: Event) {
        let mut handler = self.event_handler.borrow_mut();
        if let Some(handler) = &mut *handler {
            handler(event);
        }
    }

    /// TODO internal?
    pub fn set_event_handler(&self, handler: Box<dyn FnMut(Event)>) {
        *self.event_handler.borrow_mut() = Some(handler);
    }

    /// TODO internal?
    pub fn clear_event_handler(&self) {
        self.event_handler.borrow_mut().take();
    }

    // #[cfg(not(target_arch = "wasm32"))]
    // pub fn show(&self) {
    //     self.glutin_window.window().set_visible(true);
    // }

    /// TODO internal
    pub fn swap_buffers(&self) {
        // ugli::sync();
        let pressed_keys = self.pressed_keys.clone();
        let pressed_buttons = self.pressed_buttons.clone();
        let event_handler = self.event_handler.clone();
        self.platform.swap_buffers(move |event| {
            Self::default_handler(&event, &pressed_keys, &pressed_buttons);
            if let Some(ref mut handler) = *event_handler.borrow_mut() {
                handler(event);
            }
        });
    }

    fn default_handler(
        event: &Event,
        pressed_keys: &RefCell<HashSet<Key>>,
        pressed_buttons: &RefCell<HashSet<MouseButton>>,
    ) {
        match *event {
            Event::KeyDown { key } => {
                pressed_keys.borrow_mut().insert(key);
            }
            Event::KeyUp { key } => {
                pressed_keys.borrow_mut().remove(&key);
            }
            Event::MouseDown { button, .. } => {
                pressed_buttons.borrow_mut().insert(button);
            }
            Event::MouseUp { button, .. } => {
                pressed_buttons.borrow_mut().remove(&button);
            }
            _ => {}
        }
    }

    pub fn real_size(&self) -> vec2<usize> {
        self.platform.real_size()
    }
    pub fn size(&self) -> vec2<usize> {
        self.real_size().map(|x| x.max(1))
    }

    pub fn ugli(&self) -> &Ugli {
        let ugli = self.platform.ugli();
        ugli._set_size(self.size());
        ugli
    }

    pub fn should_close(&self) -> bool {
        self.platform.should_close()
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.borrow().contains(&key)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_buttons.borrow().contains(&button)
    }

    pub fn pressed_keys(&self) -> HashSet<Key> {
        self.pressed_keys.borrow().clone()
    }

    pub fn pressed_buttons(&self) -> HashSet<MouseButton> {
        self.pressed_buttons.borrow().clone()
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        self.platform.set_fullscreen(fullscreen);
    }

    pub fn is_fullscreen(&self) -> bool {
        self.platform.is_fullscreen()
    }

    pub fn toggle_fullscreen(&self) {
        self.set_fullscreen(!self.is_fullscreen());
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_icon(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        self.platform.set_icon(path.as_ref())
    }
}
