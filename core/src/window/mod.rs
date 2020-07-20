use crate::*;

mod cursor;
mod events;

pub use cursor::*;
pub use events::*;

pub struct Window {
    #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    canvas: stdweb::web::html_element::CanvasElement,
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    glutin_window: glutin::WindowedContext<glutin::PossiblyCurrent>,
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    glutin_event_loop: RefCell<glutin::event_loop::EventLoop<()>>,
    event_handler: Rc<RefCell<Option<Box<dyn FnMut(Event)>>>>,
    pressed_keys: Rc<RefCell<HashSet<Key>>>,
    pressed_buttons: Rc<RefCell<HashSet<MouseButton>>>,
    should_close: Cell<bool>,
    mouse_pos: Rc<Cell<Vec2<f64>>>,
    ugli: Rc<Ugli>,
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    is_fullscreen: Cell<bool>,
}

impl Window {
    pub fn new(title: &str, vsync: bool) -> Self {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        let window = {
            let _ = title;
            let _ = vsync;
            use stdweb::web::IParentNode;
            let canvas: stdweb::web::html_element::CanvasElement =
                stdweb::unstable::TryInto::try_into(
                    stdweb::web::document()
                        .query_selector("#geng-canvas")
                        .unwrap()
                        .expect("#geng-canvas not found"),
                )
                .unwrap();
            js! {
                @(no_return)
                var canvas = @{&canvas};
                window.GENG_CANVAS_SCALE = 1.0;
                canvas.tabIndex = -1;
                function update() {
                    canvas.width = Math.floor(canvas.clientWidth * window.GENG_CANVAS_SCALE);
                    canvas.height = Math.floor(canvas.clientHeight * window.GENG_CANVAS_SCALE);

                    var document = window.document;
                    if (document.fullscreenElement ||
                            document.mozFullScreenElement ||
                            document.webkitFullscreenElement ||
                            document.msFullscreenElement) {
                        screen.lockOrientationUniversal = screen.lockOrientation || screen.mozLockOrientation || screen.msLockOrientation;
                        if (screen.lockOrientationUniversal) {
                            screen.lockOrientationUniversal("landscape");
                        } else {
                            try {
                                screen.orientation.lock("landscape").catch(function () {
                                });
                            } catch (e) {}
                        }
                    } else {
                        screen.unlockOrientationUniversal = screen.unlockOrientation || screen.mozUnlockOrientation || screen.msUnlockOrientation;
                        if (screen.unlockOrientationUniversal) {
                            screen.unlockOrientationUniversal();
                        } else {
                            try {
                                screen.orientation.unlock();
                            } catch (e) {}
                        }
                    }
                };
                window.setInterval(update, 100);
                update();
            }
            let ugli = Rc::new(Ugli::create_webgl(canvas.clone()));
            let window = Self {
                canvas,
                event_handler: Rc::new(RefCell::new(None)),
                ugli,
                should_close: Cell::new(false),
                mouse_pos: Rc::new(Cell::new(vec2(0.0, 0.0))),
                pressed_keys: Rc::new(RefCell::new(HashSet::new())),
                pressed_buttons: Rc::new(RefCell::new(HashSet::new())),
            };
            let event_handler = window.event_handler.clone();
            let pressed_keys = window.pressed_keys.clone();
            let pressed_buttons = window.pressed_buttons.clone();
            let mouse_pos = window.mouse_pos.clone();
            window.subscribe_events(move |event| {
                Self::default_handler(&event, &pressed_keys, &pressed_buttons, &mouse_pos);
                if let Some(ref mut handler) = *event_handler.borrow_mut() {
                    handler(event);
                }
            });
            window
        };
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        let window = {
            let glutin_event_loop = glutin::event_loop::EventLoop::<()>::new();
            // glutin::ContextBuilder::new(),
            let glutin_window = glutin::ContextBuilder::new()
                .with_vsync(vsync)
                .build_windowed(
                    glutin::window::WindowBuilder::new().with_title(title), //.with_visibility(false)
                    &glutin_event_loop,
                )
                .unwrap();
            let glutin_window = unsafe { glutin_window.make_current() }.unwrap();
            let ugli = Rc::new(Ugli::create_from_glutin(&glutin_window));
            Self {
                glutin_window,
                glutin_event_loop: RefCell::new(glutin_event_loop),
                event_handler: Rc::new(RefCell::new(None)),
                ugli,
                should_close: Cell::new(false),
                mouse_pos: Rc::new(Cell::new(vec2(0.0, 0.0))),
                pressed_keys: Rc::new(RefCell::new(HashSet::new())),
                pressed_buttons: Rc::new(RefCell::new(HashSet::new())),
                is_fullscreen: Cell::new(false),
            }
        };
        window
    }

    pub(crate) fn set_event_handler(&self, handler: Box<dyn FnMut(Event)>) {
        *self.event_handler.borrow_mut() = Some(handler);
    }

    // #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    // pub fn show(&self) {
    //     self.glutin_window.window().set_visible(true);
    // }

    pub fn swap_buffers(&self) {
        // ugli::sync();
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        {
            self.glutin_window.swap_buffers().unwrap();
        }
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        for event in self.internal_get_events() {
            Self::default_handler(
                &event,
                &self.pressed_keys,
                &self.pressed_buttons,
                &self.mouse_pos,
            );
            if let Some(ref mut handler) = *self.event_handler.borrow_mut() {
                handler(event);
            }
        }
    }

    fn default_handler(
        event: &Event,
        pressed_keys: &RefCell<HashSet<Key>>,
        pressed_buttons: &RefCell<HashSet<MouseButton>>,
        mouse_pos: &Cell<Vec2<f64>>,
    ) {
        match *event {
            Event::KeyDown { key } => {
                pressed_keys.borrow_mut().insert(key);
            }
            Event::KeyUp { key } => {
                pressed_keys.borrow_mut().remove(&key);
            }
            Event::MouseMove { position } => {
                mouse_pos.set(position);
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

    pub fn real_size(&self) -> Vec2<usize> {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        return {
            let width = self.canvas.width() as usize;
            let height = self.canvas.height() as usize;
            vec2(width, height)
        };
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        return {
            let size = self.glutin_window.window().inner_size();
            let (width, height) = (size.width, size.height);
            vec2(width as usize, height as usize)
        };
    }
    pub fn size(&self) -> Vec2<usize> {
        self.real_size().map(|x| x.max(1))
    }

    pub fn ugli(&self) -> &Rc<Ugli> {
        self.ugli._set_size(self.size());
        &self.ugli
    }

    pub fn should_close(&self) -> bool {
        self.should_close.get()
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

    pub fn mouse_pos(&self) -> Vec2<f64> {
        self.mouse_pos.get()
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        {
            if fullscreen {
                js! {
                    var elem = @{&self.canvas};
                    if (elem.requestFullscreen) {
                        elem.requestFullscreen();
                    } else if (elem.msRequestFullscreen) {
                        elem.msRequestFullscreen();
                    } else if (elem.mozRequestFullScreen) {
                        elem.mozRequestFullScreen();
                    } else if (elem.webkitRequestFullscreen) {
                        elem.webkitRequestFullscreen();
                    }
                }
            } else {
                js! {
                    var document = window.document;
                    if (document.cancelFullScreen) {
                        document.cancelFullScreen();
                    } else if (document.msExitFullscreen) {
                        document.msExitFullscreen();
                    } else if (document.mozCancelFullScreen) {
                        document.mozCancelFullScreen();
                    } else if (document.webkitCancelFullScreen) {
                        document.webkitCancelFullScreen();
                    }
                }
            }
        }
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        {
            self.glutin_window.window().set_fullscreen(if fullscreen {
                Some(glutin::window::Fullscreen::Borderless(
                    self.glutin_window.window().current_monitor(),
                ))
            } else {
                None
            });
            self.is_fullscreen.set(fullscreen);
        }
    }

    pub fn is_fullscreen(&self) -> bool {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        {
            return stdweb::unstable::TryInto::try_into(js! {
                var document = window.document;
                if (document.fullscreenElement ||
                    document.mozFullScreenElement ||
                    document.webkitFullscreenElement ||
                    document.msFullscreenElement) {
                        return true;
                } else {
                    return false;
                }
            })
            .unwrap();
        }
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        self.is_fullscreen.get()
    }

    pub fn toggle_fullscreen(&self) {
        self.set_fullscreen(!self.is_fullscreen());
    }
}
