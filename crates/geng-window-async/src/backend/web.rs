use super::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/platform/web.js")]
extern "C" {
    pub fn initialize_window(canvas: &web_sys::HtmlCanvasElement);
    pub fn is_fullscreen() -> bool;
    pub fn set_fullscreen(canvas: &web_sys::HtmlCanvasElement, fullscreen: bool);
}

pub struct Context {
    canvas: web_sys::HtmlCanvasElement,
    ugli: Ugli,
    mouse_pos: Rc<Cell<vec2<f64>>>,
    #[allow(clippy::type_complexity)]
    event_handler: Rc<RefCell<Box<dyn Fn(Event)>>>,
    editing_text: Rc<Cell<bool>>,
    text_agent: web_sys::HtmlInputElement,
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
        let event_handler = Rc::new(RefCell::new(Box::new(|_| {}) as Box<_>));
        let context = Self {
            canvas,
            ugli,
            event_handler: event_handler.clone(),
            mouse_pos: Rc::new(Cell::new(vec2::ZERO)),
            editing_text: Rc::new(Cell::new(false)),
            text_agent: Self::install_text_agent().unwrap(),
        };
        context.subscribe_events(move |event| {
            event_handler.borrow()(event);
        });
        context
    }

    pub fn real_size(&self) -> vec2<usize> {
        let width = self.canvas.width() as usize;
        let height = self.canvas.height() as usize;
        vec2(width, height)
    }

    pub fn ugli(&self) -> &Ugli {
        &self.ugli
    }

    pub fn should_close(&self) -> bool {
        false
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        set_fullscreen(&self.canvas, fullscreen);
    }

    pub fn is_fullscreen(&self) -> bool {
        is_fullscreen()
    }

    pub fn mouse_pos(&self) -> vec2<f64> {
        self.mouse_pos.get()
    }

    pub fn set_cursor_position(&self, _: vec2<f64>) {
        unimplemented!()
    }

    pub fn swap_buffers(&self, event_handler: impl Fn(Event) + 'static) {
        let mouse_pos = self.mouse_pos.clone();
        *self.event_handler.borrow_mut() = Box::new(move |event| {
            if let Event::MouseMove { position, .. } = event {
                mouse_pos.set(position);
            }
            event_handler(event);
        });
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
}

trait Convert<T>: Sized {
    fn convert(value: T) -> Option<Self>;
}

trait ConvertEvent<T>: Sized {
    fn convert(value: T) -> Vec<Self>;
}

impl Convert<String> for Key {
    fn convert(key: String) -> Option<Key> {
        use Key::*;
        Some(match key.as_str() {
            "KeyA" => A,
            "KeyB" => B,
            "KeyC" => C,
            "KeyD" => D,
            "KeyE" => E,
            "KeyF" => F,
            "KeyG" => G,
            "KeyH" => H,
            "KeyI" => I,
            "KeyJ" => J,
            "KeyK" => K,
            "KeyL" => L,
            "KeyM" => M,
            "KeyN" => N,
            "KeyO" => O,
            "KeyP" => P,
            "KeyQ" => Q,
            "KeyR" => R,
            "KeyS" => S,
            "KeyT" => T,
            "KeyU" => U,
            "KeyV" => V,
            "KeyW" => W,
            "KeyX" => X,
            "KeyY" => Y,
            "KeyZ" => Z,

            "Digit0" => Num0,
            "Digit1" => Num1,
            "Digit2" => Num2,
            "Digit3" => Num3,
            "Digit4" => Num4,
            "Digit5" => Num5,
            "Digit6" => Num6,
            "Digit7" => Num7,
            "Digit8" => Num8,
            "Digit9" => Num9,

            "Escape" => Escape,
            "Space" => Space,
            "Enter" => Enter,
            "Backspace" => Backspace,
            "Tab" => Tab,

            "ShiftLeft" => LShift,
            "ShiftRight" => RShift,

            "ControlLeft" => LCtrl,
            "ControlRight" => RCtrl,

            "AltLeft" => LAlt,
            "AltRight" => RAlt,

            "ArrowLeft" => Left,
            "ArrowRight" => Right,
            "ArrowUp" => Up,
            "ArrowDown" => Down,

            "PageUp" => PageUp,
            "PageDown" => PageDown,
            "Insert" => Insert,
            "Delete" => Delete,
            "Home" => Home,
            "End" => End,

            "F1" => F1,
            "F2" => F2,
            "F3" => F3,
            "F4" => F4,
            "F5" => F5,
            "F6" => F6,
            "F7" => F7,
            "F8" => F8,
            "F9" => F9,
            "F10" => F10,
            "F11" => F11,
            "F12" => F12,

            _ => {
                log::warn!("Unrecognized key: {:?}", key);
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
        let Some(key) = Convert::convert(event.code()) else { return vec![] };
        vec![match event.type_().as_str() {
            "keydown" => Event::KeyDown { key },
            "keyup" => Event::KeyUp { key },
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
                "mousedown" => Event::MouseDown {
                    position,
                    button: button?,
                },
                "mouseup" => Event::MouseUp {
                    position,
                    button: button?,
                },
                "mousemove" => Event::MouseMove {
                    position,
                    delta: vec2(event.movement_x(), -event.movement_y()).map(|x| x as f64),
                },
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
        self.subscribe_to::<web_sys::KeyboardEvent>(&self.canvas, handler, "keydown");
        self.subscribe_to::<web_sys::KeyboardEvent>(&self.canvas, handler, "keyup");
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
