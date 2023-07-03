use batbox_la::*;
use serde::{Deserialize, Serialize};
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::Rc;
use ugli::Ugli;

mod backend;

mod cursor;
mod events;

pub use cursor::*;
pub use events::*;

#[derive(Debug, Clone, Serialize, Deserialize, clap::Args, Default)]
#[group(id = "window")]
pub struct CliArgs {
    /// Turn vertical synchronization on/off
    #[clap(long, value_name = "BOOL")]
    pub vsync: Option<bool>,
    /// Turn antialiasing on/off
    #[clap(long, value_name = "BOOL")]
    pub antialias: Option<bool>,
    /// Start with given window width (also requires window-height)
    #[clap(long = "window-width", value_name = "PIXELS")]
    pub width: Option<usize>,
    /// Start with given window height (also requires window-width)
    #[clap(long = "window-height", value_name = "PIXELS")]
    pub height: Option<usize>,
    /// Start in fullscreen
    #[clap(long, value_name = "BOOL")]
    pub fullscreen: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub fullscreen: bool,
    pub vsync: bool,
    pub title: String,
    pub antialias: bool,
    pub transparency: bool,
    pub size: Option<vec2<usize>>,
    pub auto_close: bool,
    pub start_hidden: bool,
}

impl Options {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            fullscreen: !cfg!(debug_assertions),
            vsync: true,
            antialias: true,
            transparency: false,
            size: None,
            auto_close: true,
            start_hidden: false,
        }
    }

    pub fn with_cli(&mut self, args: &CliArgs) {
        if let Some(vsync) = args.vsync {
            self.vsync = vsync;
        }
        if let Some(antialias) = args.antialias {
            self.antialias = antialias;
        }
        if let (Some(window_width), Some(window_height)) = (args.width, args.height) {
            self.size = Some(vec2(window_width, window_height));
        }
        if let Some(fullscreen) = args.fullscreen {
            self.fullscreen = fullscreen;
        }
    }
}

struct WindowImpl {
    event_sender: async_broadcast::Sender<Event>,
    event_receiver: RefCell<async_broadcast::Receiver<Event>>,
    executor: async_executor::LocalExecutor<'static>,
    backend: Rc<backend::Context>,
    pressed_keys: Rc<RefCell<HashSet<Key>>>,
    pressed_buttons: Rc<RefCell<HashSet<MouseButton>>>,
    cursor_pos: Cell<Option<vec2<f64>>>,
    cursor_type: Cell<CursorType>,
    auto_close: Cell<bool>,
}

#[derive(Clone)]
pub struct Window {
    inner: Rc<WindowImpl>,
}

impl Window {
    pub fn start_text_edit(&self, text: &str) {
        self.inner.backend.start_text_edit(text);
    }

    pub fn stop_text_edit(&self) {
        self.inner.backend.stop_text_edit();
    }

    pub fn is_editing_text(&self) -> bool {
        self.inner.backend.is_editing_text()
    }

    pub fn real_size(&self) -> vec2<usize> {
        self.inner.backend.real_size()
    }
    pub fn size(&self) -> vec2<usize> {
        self.real_size().map(|x| x.max(1))
    }

    pub fn ugli(&self) -> &Ugli {
        self.inner.backend.ugli()
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.inner.pressed_keys.borrow().contains(&key)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.inner.pressed_buttons.borrow().contains(&button)
    }

    pub fn pressed_keys(&self) -> HashSet<Key> {
        self.inner.pressed_keys.borrow().clone()
    }

    pub fn pressed_buttons(&self) -> HashSet<MouseButton> {
        self.inner.pressed_buttons.borrow().clone()
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        self.inner.backend.set_fullscreen(fullscreen);
    }

    pub fn is_fullscreen(&self) -> bool {
        self.inner.backend.is_fullscreen()
    }

    pub fn toggle_fullscreen(&self) {
        self.set_fullscreen(!self.is_fullscreen());
    }

    pub fn set_auto_close(&self, auto_close: bool) {
        self.inner.auto_close.set(auto_close);
    }

    pub fn is_auto_close(&self) -> bool {
        self.inner.auto_close.get()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_icon(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        self.inner.backend.set_icon(path.as_ref())
    }

    pub fn spawn(
        &self,
        f: impl std::future::Future<Output = ()> + 'static,
    ) -> async_executor::Task<()> {
        self.inner.executor.spawn(f)
    }

    pub fn with_framebuffer(&self, f: impl FnOnce(&mut ugli::Framebuffer)) {
        self.inner.backend.with_framebuffer(f);
    }

    pub fn events(&self) -> impl futures::Stream<Item = Event> {
        self.inner.event_receiver.borrow().clone()
    }

    pub fn show(&self) {
        self.inner.backend.show();
    }
}

pub fn run<Fut>(options: &Options, f: impl 'static + FnOnce(Window) -> Fut)
where
    Fut: std::future::Future<Output = ()> + 'static,
{
    let options = options.clone();
    backend::run(&options, move |backend| {
        // channel capacity is 1 because events are supposed to be consumed immediately
        let (mut event_sender, event_receiver) = async_broadcast::broadcast(1);
        event_sender.set_overflow(true);
        let window = Window {
            inner: Rc::new(WindowImpl {
                event_sender,
                // We can't just not have this receiver since the channel will be closed then
                event_receiver: RefCell::new(event_receiver),
                executor: async_executor::LocalExecutor::new(),
                backend,
                pressed_keys: Rc::new(RefCell::new(HashSet::new())),
                pressed_buttons: Rc::new(RefCell::new(HashSet::new())),
                auto_close: Cell::new(options.auto_close),
                cursor_pos: Cell::new(None),
                cursor_type: Cell::new(CursorType::Default),
            }),
        };
        if options.fullscreen {
            window.set_fullscreen(true);
        }
        if !options.start_hidden {
            window.show();
        }

        let f = f(window.clone());
        let main_task = window.spawn(f);
        while window.inner.executor.try_tick() {}
        move |event| {
            match event {
                Event::KeyPress { key } => {
                    if !window.inner.pressed_keys.borrow_mut().insert(key) {
                        return std::ops::ControlFlow::Continue(());
                    }
                }
                Event::KeyRelease { key } => {
                    if !window.inner.pressed_keys.borrow_mut().remove(&key) {
                        return std::ops::ControlFlow::Continue(());
                    }
                }
                Event::MousePress { button } => {
                    window.inner.pressed_buttons.borrow_mut().insert(button);
                }
                Event::MouseRelease { button } => {
                    window.inner.pressed_buttons.borrow_mut().remove(&button);
                }
                Event::CursorMove { position } => {
                    window.inner.cursor_pos.set(Some(position));
                    if window.cursor_locked() {
                        return std::ops::ControlFlow::Continue(());
                    }
                }
                Event::RawMouseMove { .. } => {
                    if !window.cursor_locked() {
                        return std::ops::ControlFlow::Continue(());
                    }
                }
                Event::CloseRequested => {
                    if window.is_auto_close() {
                        return std::ops::ControlFlow::Break(());
                    }
                }
                _ => {}
            }
            if let Some(_removed) = window.inner.event_sender.try_broadcast(event).unwrap() {
                // log::error!("Event has been ignored: {removed:?}");
            }
            window.inner.event_receiver.borrow_mut().try_recv().unwrap();
            while window.inner.executor.try_tick() {
                if main_task.is_finished() {
                    return std::ops::ControlFlow::Break(());
                }
            }
            std::ops::ControlFlow::Continue(())
        }
    });
}
