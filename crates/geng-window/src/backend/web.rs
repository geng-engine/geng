use super::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/backend/web.js")]
extern "C" {
    fn initialize_window(canvas: &web_sys::HtmlCanvasElement);
    fn is_fullscreen() -> bool;
    fn set_fullscreen(canvas: &web_sys::HtmlCanvasElement, fullscreen: bool);
    fn show();
    fn request_animation_frame_loop(closure: &Closure<dyn FnMut()>);
}

pub struct Context {
    canvas: web_sys::HtmlCanvasElement,
    ugli: Ugli,
    editing_text: Rc<Cell<bool>>,
    text_agent: web_sys::HtmlInputElement,
}

pub fn run<EH>(options: &Options, once_ready: impl 'static + FnOnce(Rc<Context>) -> EH)
where
    EH: 'static + FnMut(Event) -> std::ops::ControlFlow<()>,
{
    let context = Rc::new(Context::new(options));
    let event_handler = once_ready(context.clone());
    context.run(event_handler);
}

impl Context {
    pub fn new(options: &Options) -> Self {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("geng-canvas")
            .expect("#geng-canvas not found")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("#geng-canvas is not a canvas");
        initialize_window(&canvas);
        let ugli = Ugli::create_webgl(
            &canvas,
            ugli::WebGLContextOptions {
                antialias: options.antialias,
                alpha: options.transparency,
                stencil: true,
                ..Default::default()
            },
        );
        Self {
            canvas,
            ugli,
            editing_text: Rc::new(Cell::new(false)),
            text_agent: Self::install_text_agent().unwrap(),
        }
    }

    pub fn real_size(&self) -> vec2<usize> {
        let width = self.canvas.width() as usize;
        let height = self.canvas.height() as usize;
        vec2(width, height)
    }

    pub fn ugli(&self) -> &Ugli {
        &self.ugli
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        set_fullscreen(&self.canvas, fullscreen);
    }

    pub fn is_fullscreen(&self) -> bool {
        is_fullscreen()
    }

    pub fn set_cursor_type(&self, cursor_type: CursorType) {
        let cursor_type = match cursor_type {
            CursorType::Default => "initial",
            CursorType::Pointer => "pointer",
            CursorType::Drag => "all-scroll",
            CursorType::None => "none",
        };
        // TODO: only canvas
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .style()
            .set_property("cursor", cursor_type)
            .unwrap();
    }

    pub fn cursor_locked(&self) -> bool {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .pointer_lock_element()
            .is_some()
    }

    pub fn lock_cursor(&self) {
        self.canvas.request_pointer_lock();
    }

    pub fn unlock_cursor(&self) {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .exit_pointer_lock();
    }

    pub fn start_text_edit(&self, text: &str) {
        self.editing_text.set(true);
        self.text_agent.set_value(text);
        self.update_text_agent();
    }

    pub fn stop_text_edit(&self) {
        self.editing_text.set(false);
        self.update_text_agent();
    }

    pub fn is_editing_text(&self) -> bool {
        self.editing_text.get()
    }

    pub fn run(
        self: Rc<Self>,
        event_handler: impl FnMut(Event) -> std::ops::ControlFlow<()> + 'static,
    ) {
        let event_handler = RefCell::new(event_handler);
        self.subscribe_events(move |event| {
            if let std::ops::ControlFlow::Break(()) = (event_handler.borrow_mut())(event) {
                panic!("Should not be exiting one the web!");
            }
        });
    }

    pub fn with_framebuffer<T>(&self, f: impl FnOnce(&mut ugli::Framebuffer) -> T) -> T {
        f(&mut ugli::Framebuffer::default(
            &self.ugli,
            self.real_size(),
        ))
    }

    pub fn show(&self) {
        show();
    }
}

trait Convert<T>: Sized {
    fn convert(value: T) -> Option<Self>;
}

trait ConvertEvent<T>: Sized {
    fn convert(value: T) -> Vec<Self>;
}

impl Convert<String> for Key {
    fn convert(key: String) -> Option<Key> {
        Some(match key.as_str() {
            "Backquote" => Key::Backquote,
            "Backslash" => Key::Backslash,
            "BracketLeft" => Key::BracketLeft,
            "BracketRight" => Key::BracketRight,
            "Comma" => Key::Comma,
            "Digit0" => Key::Digit0,
            "Digit1" => Key::Digit1,
            "Digit2" => Key::Digit2,
            "Digit3" => Key::Digit3,
            "Digit4" => Key::Digit4,
            "Digit5" => Key::Digit5,
            "Digit6" => Key::Digit6,
            "Digit7" => Key::Digit7,
            "Digit8" => Key::Digit8,
            "Digit9" => Key::Digit9,
            "Equal" => Key::Equal,
            "IntlBackslash" => Key::IntlBackslash,
            "IntlRo" => Key::IntlRo,
            "IntlYen" => Key::IntlYen,
            "KeyA" => Key::A,
            "KeyB" => Key::B,
            "KeyC" => Key::C,
            "KeyD" => Key::D,
            "KeyE" => Key::E,
            "KeyF" => Key::F,
            "KeyG" => Key::G,
            "KeyH" => Key::H,
            "KeyI" => Key::I,
            "KeyJ" => Key::J,
            "KeyK" => Key::K,
            "KeyL" => Key::L,
            "KeyM" => Key::M,
            "KeyN" => Key::N,
            "KeyO" => Key::O,
            "KeyP" => Key::P,
            "KeyQ" => Key::Q,
            "KeyR" => Key::R,
            "KeyS" => Key::S,
            "KeyT" => Key::T,
            "KeyU" => Key::U,
            "KeyV" => Key::V,
            "KeyW" => Key::W,
            "KeyX" => Key::X,
            "KeyY" => Key::Y,
            "KeyZ" => Key::Z,
            "Minus" => Key::Minus,
            "Period" => Key::Period,
            "Quote" => Key::Quote,
            "Semicolon" => Key::Semicolon,
            "Slash" => Key::Slash,
            "AltLeft" => Key::AltLeft,
            "AltRight" => Key::AltRight,
            "Backspace" => Key::Backspace,
            "CapsLock" => Key::CapsLock,
            "ContextMenu" => Key::ContextMenu,
            "ControlLeft" => Key::ControlLeft,
            "ControlRight" => Key::ControlRight,
            "Enter" => Key::Enter,
            "SuperLeft" => Key::SuperLeft,
            "SuperRight" => Key::SuperRight,
            "ShiftLeft" => Key::ShiftLeft,
            "ShiftRight" => Key::ShiftRight,
            "Space" => Key::Space,
            "Tab" => Key::Tab,
            "Delete" => Key::Delete,
            "End" => Key::End,
            "Help" => Key::Help,
            "Home" => Key::Home,
            "Insert" => Key::Insert,
            "PageDown" => Key::PageDown,
            "PageUp" => Key::PageUp,
            "ArrowDown" => Key::ArrowDown,
            "ArrowLeft" => Key::ArrowLeft,
            "ArrowRight" => Key::ArrowRight,
            "ArrowUp" => Key::ArrowUp,
            "NumLock" => Key::NumLock,
            "Numpad0" => Key::Numpad0,
            "Numpad1" => Key::Numpad1,
            "Numpad2" => Key::Numpad2,
            "Numpad3" => Key::Numpad3,
            "Numpad4" => Key::Numpad4,
            "Numpad5" => Key::Numpad5,
            "Numpad6" => Key::Numpad6,
            "Numpad7" => Key::Numpad7,
            "Numpad8" => Key::Numpad8,
            "Numpad9" => Key::Numpad9,
            "NumpadAdd" => Key::NumpadAdd,
            "NumpadBackspace" => Key::NumpadBackspace,
            "NumpadClear" => Key::NumpadClear,
            "NumpadClearEntry" => Key::NumpadClearEntry,
            "NumpadComma" => Key::NumpadComma,
            "NumpadDecimal" => Key::NumpadDecimal,
            "NumpadDivide" => Key::NumpadDivide,
            "NumpadEnter" => Key::NumpadEnter,
            "NumpadEqual" => Key::NumpadEqual,
            "NumpadHash" => Key::NumpadHash,
            "NumpadMemoryAdd" => Key::NumpadMemoryAdd,
            "NumpadMemoryClear" => Key::NumpadMemoryClear,
            "NumpadMemoryRecall" => Key::NumpadMemoryRecall,
            "NumpadMemoryStore" => Key::NumpadMemoryStore,
            "NumpadMemorySubtract" => Key::NumpadMemorySubtract,
            "NumpadMultiply" => Key::NumpadMultiply,
            "NumpadParenLeft" => Key::NumpadParenLeft,
            "NumpadParenRight" => Key::NumpadParenRight,
            "NumpadStar" => Key::NumpadStar,
            "NumpadSubtract" => Key::NumpadSubtract,
            "Escape" => Key::Escape,
            "BrowserBack" => Key::Back,
            "F1" => Key::F1,
            "F2" => Key::F2,
            "F3" => Key::F3,
            "F4" => Key::F4,
            "F5" => Key::F5,
            "F6" => Key::F6,
            "F7" => Key::F7,
            "F8" => Key::F8,
            "F9" => Key::F9,
            "F10" => Key::F10,
            "F11" => Key::F11,
            "F12" => Key::F12,

            _ => {
                log::trace!("Unrecognized key: {:?}", key);
                return None;
            }
        })
    }
}

impl Convert<i16> for MouseButton {
    fn convert(button: i16) -> Option<MouseButton> {
        Some(match button {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            // 3 => MouseButton::Back,
            // 4 => MouseButton::Forward,
            _ => return None,
        })
    }
}

impl ConvertEvent<web_sys::KeyboardEvent> for Event {
    fn convert(event: web_sys::KeyboardEvent) -> Vec<Event> {
        if event.repeat() {
            return vec![];
        }
        let Some(key) = Convert::convert(event.code()) else {
            return vec![];
        };
        vec![match event.type_().as_str() {
            "keydown" => Event::KeyPress { key },
            "keyup" => Event::KeyRelease { key },
            _ => return vec![],
        }]
    }
}

impl ConvertEvent<web_sys::MouseEvent> for Event {
    fn convert(event: web_sys::MouseEvent) -> Vec<Event> {
        let event = || -> Option<Event> {
            let button = Convert::convert(event.button());
            let canvas: web_sys::HtmlCanvasElement = event.target().unwrap().dyn_into().unwrap();
            let position = vec2(
                event.offset_x(),
                canvas.height() as i32 - 1 - event.offset_y(),
            )
            .map(|x| x as f64);
            Some(match event.type_().as_str() {
                "mousedown" => Event::MousePress { button: button? },
                "mouseup" => Event::MouseRelease { button: button? },
                "mousemove" => {
                    let cursor_locked = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .pointer_lock_element()
                        .is_some();
                    if cursor_locked {
                        let delta = vec2(event.movement_x(), -event.movement_y()).map(|x| x as f64);
                        // KEKW BROWSERS SUCK
                        const MAX: f64 = 50.0;
                        if delta.x.abs() > MAX || delta.y.abs() > MAX {
                            return None;
                        }

                        Event::RawMouseMove { delta }
                    } else {
                        Event::CursorMove { position }
                    }
                }
                _ => return None,
            })
        };
        event().into_iter().collect()
    }
}

impl ConvertEvent<web_sys::WheelEvent> for Event {
    fn convert(event: web_sys::WheelEvent) -> Vec<Event> {
        vec![Event::Wheel {
            delta: -event.delta_y()
                * match event.delta_mode() {
                    web_sys::WheelEvent::DOM_DELTA_PIXEL => 1.0,
                    web_sys::WheelEvent::DOM_DELTA_LINE => 51.0,
                    web_sys::WheelEvent::DOM_DELTA_PAGE => 800.0,
                    _ => {
                        log::error!("Unexpected delta mode: {}", event.delta_mode());
                        return vec![];
                    }
                },
        }]
    }
}

impl ConvertEvent<web_sys::TouchEvent> for Event {
    fn convert(event: web_sys::TouchEvent) -> Vec<Event> {
        let create_event: Box<dyn Fn(Touch) -> Event> = match event.type_().as_str() {
            "touchstart" => Box::new(Event::TouchStart),
            "touchmove" => Box::new(Event::TouchMove),
            "touchcancel" | "touchend" => Box::new(Event::TouchEnd),
            _ => return vec![],
        };
        let canvas: web_sys::HtmlCanvasElement = event.target().unwrap().dyn_into().unwrap();
        let rect = canvas.get_bounding_client_rect();
        let touches = event.changed_touches();
        (0..touches.length())
            .map(|index| {
                let touch = touches.item(index).unwrap();
                let offset_x = touch.page_x() as f64 - rect.left();
                let offset_y = touch.page_y() as f64 - rect.top();
                create_event(Touch {
                    id: touch.identifier() as u64,
                    position: vec2(offset_x, canvas.height() as f64 - 1.0 - offset_y),
                })
            })
            .collect()
    }
}

const TEXT_AGENT_PREFIX: &str = "💩";

impl Context {
    fn subscribe_to_raw<T>(
        &self,
        target: &web_sys::EventTarget,
        handler: impl Fn(T) + 'static,
        event_name: &str,
    ) where
        T: wasm_bindgen::convert::FromWasmAbi + 'static,
        T: AsRef<web_sys::Event>,
        T: Clone,
    {
        let handler = wasm_bindgen::closure::Closure::wrap(Box::new(handler) as Box<dyn Fn(T)>);
        target
            .add_event_listener_with_callback(event_name, handler.as_ref().unchecked_ref())
            .unwrap();
        handler.forget(); // TODO: not forget
    }
    fn subscribe_to<T>(
        &self,
        target: &web_sys::EventTarget,
        handler: &Rc<impl Fn(Event) + 'static>,
        event_name: &str,
    ) where
        T: wasm_bindgen::convert::FromWasmAbi + 'static,
        T: AsRef<web_sys::Event>,
        T: Clone,
        Event: ConvertEvent<T>,
    {
        let handler = handler.clone();
        let canvas = self.canvas.clone();
        let text_agent = self.text_agent.clone();
        let editing_text = self.editing_text.clone();
        let handler = move |event: T| {
            if editing_text.get() {
                text_agent.focus().unwrap();
            } else {
                canvas.focus().unwrap();
            }
            if event.as_ref().type_() == "contextmenu" {
                event.as_ref().prevent_default();
            }
            for e in ConvertEvent::convert(event.clone()) {
                handler(e);
                event.as_ref().prevent_default();
            }
        };
        self.subscribe_to_raw(target, handler, event_name);
    }
    fn subscribe_events<F: Fn(Event) + 'static>(&self, handler: F) {
        let handler = Rc::new(handler);
        let handler = &handler;

        // TODO: check - did for salmoning
        let document = web_sys::window().unwrap().document().unwrap();
        let event_target = &document; // &self.canvas;

        self.subscribe_to::<web_sys::KeyboardEvent>(event_target, handler, "keydown");
        self.subscribe_to::<web_sys::KeyboardEvent>(event_target, handler, "keyup");
        self.subscribe_to_raw::<web_sys::InputEvent>(
            &self.text_agent,
            {
                let handler = handler.clone();
                move |event: web_sys::InputEvent| {
                    let input: web_sys::HtmlInputElement =
                        event.target().unwrap().dyn_into().unwrap();
                    handler(Event::EditText(input.value()));
                }
            },
            "input",
        );
        self.subscribe_to::<web_sys::MouseEvent>(&self.canvas, handler, "mousedown");
        self.subscribe_to::<web_sys::MouseEvent>(&self.canvas, handler, "mouseup");
        self.subscribe_to::<web_sys::MouseEvent>(&self.canvas, handler, "mousemove");
        self.subscribe_to::<web_sys::WheelEvent>(&self.canvas, handler, "wheel");
        self.subscribe_to::<web_sys::TouchEvent>(&self.canvas, handler, "touchstart");
        self.subscribe_to::<web_sys::TouchEvent>(&self.canvas, handler, "touchmove");
        self.subscribe_to::<web_sys::TouchEvent>(&self.canvas, handler, "touchend");
        self.subscribe_to::<web_sys::TouchEvent>(&self.canvas, handler, "touchcancel");
        self.subscribe_to::<web_sys::MouseEvent>(&self.canvas, handler, "contextmenu");
        {
            let handler = handler.clone();
            let closure =
                wasm_bindgen::closure::Closure::wrap(
                    Box::new(move || handler(Event::Draw)) as Box<dyn FnMut()>
                );
            request_animation_frame_loop(&closure);
            std::mem::forget(closure); // Don't drop so that JS can call this thing
        };
    }

    fn install_text_agent() -> Result<web_sys::HtmlInputElement, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().expect("document should have a body");
        let input = document
            .create_element("input")?
            .dyn_into::<web_sys::HtmlInputElement>()?;
        {
            let style = input.style();
            // Transparent
            style.set_property("opacity", "0").unwrap();
            // Hide under canvas
            style.set_property("z-index", "-1").unwrap();
            // z-index doesn't work otherwise
            style.set_property("position", "absolute").unwrap();
            style.set_property("top", "0").unwrap();
        }
        // Set size as small as possible, in case user may click on it.
        input.set_size(1);
        input.set_autofocus(true);
        input.set_hidden(true);
        input.set_value(TEXT_AGENT_PREFIX);

        body.append_child(&input)?;
        Ok(input)
    }

    /// Focus or blur text agent to toggle mobile keyboard.
    fn update_text_agent(&self) {
        if self.editing_text.get() {
            self.text_agent.set_hidden(false);
            self.text_agent.focus().unwrap();
        } else {
            self.text_agent.set_hidden(true);
            self.canvas.focus().unwrap();
        }
    }
}
