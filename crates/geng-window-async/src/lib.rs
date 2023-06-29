use batbox_la::*;
use serde::{Deserialize, Serialize};
use std::cell::{Cell, RefCell, RefMut};
use std::collections::HashSet;
use std::rc::Rc;
use ugli::Ugli;

mod platform;

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
        }
    }

    pub fn with_cli(mut self, args: &CliArgs) -> Self {
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
        self
    }
}

struct WindowImpl {
    event_sender: async_broadcast::Sender<Event>,
    event_receiver: RefCell<async_broadcast::Receiver<Event>>,
    executor: async_executor::LocalExecutor<'static>,
    platform: Rc<platform::Context>,
    #[allow(clippy::type_complexity)]
    event_handler: Rc<RefCell<Option<Box<dyn FnMut(Event)>>>>,
    pressed_keys: Rc<RefCell<HashSet<Key>>>,
    pressed_buttons: Rc<RefCell<HashSet<MouseButton>>>,
    auto_close: Cell<bool>,
}

#[derive(Clone)]
pub struct Window {
    inner: Rc<WindowImpl>,
}

impl Window {
    fn new(options: &Options) -> Self {
        // channel capacity is 1 because events are supposed to be consumed immediately
        let (mut event_sender, event_receiver) = async_broadcast::broadcast(1);
        event_sender.set_overflow(true);
        let window = Self {
            inner: Rc::new(WindowImpl {
                event_sender,
                // We can't just not have this receiver since the channel will be closed then
                event_receiver: RefCell::new(event_receiver),
                executor: async_executor::LocalExecutor::new(),
                platform: Rc::new(platform::Context::new(options)),
                event_handler: Rc::new(RefCell::new(None)),
                pressed_keys: Rc::new(RefCell::new(HashSet::new())),
                pressed_buttons: Rc::new(RefCell::new(HashSet::new())),
                auto_close: Cell::new(options.auto_close),
            }),
        };
        if options.fullscreen {
            window.set_fullscreen(true);
        }
        window
    }

    pub fn start_text_edit(&self, text: &str) {
        self.inner.platform.start_text_edit(text);
    }

    pub fn stop_text_edit(&self) {
        self.inner.platform.stop_text_edit();
    }

    pub fn real_size(&self) -> vec2<usize> {
        self.inner.platform.real_size()
    }
    pub fn size(&self) -> vec2<usize> {
        self.real_size().map(|x| x.max(1))
    }

    pub fn ugli(&self) -> &Ugli {
        let ugli = self.inner.platform.ugli();
        ugli._set_size(self.size());
        ugli
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
        self.inner.platform.set_fullscreen(fullscreen);
    }

    pub fn is_fullscreen(&self) -> bool {
        self.inner.platform.is_fullscreen()
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
        self.inner.platform.set_icon(path.as_ref())
    }

    pub fn spawn(
        &self,
        f: impl std::future::Future<Output = ()> + 'static,
    ) -> async_executor::Task<()> {
        self.inner.executor.spawn(f)
    }

    pub fn with_framebuffer(&self, f: impl FnOnce(&mut ugli::Framebuffer)) {
        f(&mut self.inner.platform.lock_framebuffer());
    }

    pub fn events(&self) -> impl futures::Stream<Item = Event> {
        self.inner.event_receiver.borrow().clone()
    }
}

pub fn run<Fut>(options: Options, f: impl FnOnce(Window) -> Fut)
where
    Fut: std::future::Future<Output = ()> + 'static,
{
    let this = Window::new(&options);
    let f = f(this.clone());
    let main_task = this.spawn(f);
    this.inner.platform.clone().run(move |event| {
        if let Event::CloseRequested = event {
            if this.is_auto_close() {
                return std::ops::ControlFlow::Break(());
            }
        }
        if let Some(_removed) = this.inner.event_sender.try_broadcast(event).unwrap() {
            // log::error!("Event has been ignored: {removed:?}");
        }
        this.inner.event_receiver.borrow_mut().try_recv().unwrap();
        while this.inner.executor.try_tick() {
            if main_task.is_finished() {
                return std::ops::ControlFlow::Break(());
            }
        }
        std::ops::ControlFlow::Continue(())
    });
}
