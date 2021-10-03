use super::*;

trait Convert<T>: Sized {
    fn convert(value: T) -> Option<Self>;
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
                warn!("Unrecognized key: {:?}", key);
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

impl Convert<web_sys::KeyboardEvent> for Event {
    fn convert(event: web_sys::KeyboardEvent) -> Option<Event> {
        if event.repeat() {
            return None;
        }
        let key = Convert::convert(event.code())?;
        Some(match event.type_().as_str() {
            "keydown" => Event::KeyDown { key },
            "keyup" => Event::KeyUp { key },
            _ => return None,
        })
    }
}

impl Convert<web_sys::MouseEvent> for Event {
    fn convert(event: web_sys::MouseEvent) -> Option<Event> {
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
    }
}

impl Convert<web_sys::WheelEvent> for Event {
    fn convert(event: web_sys::WheelEvent) -> Option<Event> {
        Some(Event::Wheel {
            delta: -event.delta_y()
                * match event.delta_mode() {
                    web_sys::WheelEvent::DOM_DELTA_PIXEL => 1.0,
                    web_sys::WheelEvent::DOM_DELTA_LINE => 51.0,
                    web_sys::WheelEvent::DOM_DELTA_PAGE => 800.0,
                    _ => {
                        error!("Unexpected delta mode: {}", event.delta_mode());
                        return None;
                    }
                },
        })
    }
}

impl Convert<web_sys::TouchEvent> for Event {
    fn convert(event: web_sys::TouchEvent) -> Option<Event> {
        let canvas: web_sys::HtmlCanvasElement = event.target().unwrap().dyn_into().unwrap();
        let rect = canvas.get_bounding_client_rect();
        let touches = event.touches();
        let touches = (0..touches.length())
            .map(|index| {
                let touch = touches.item(index).unwrap();
                let offset_x = touch.page_x() as f64 - rect.left();
                let offset_y = touch.page_y() as f64 - rect.top();
                TouchPoint {
                    position: vec2(offset_x, canvas.height() as f64 - 1.0 - offset_y),
                }
            })
            .collect();
        Some(match event.type_().as_str() {
            "touchstart" => Event::TouchStart { touches },
            "touchmove" => Event::TouchMove { touches },
            "touchcancel" | "touchend" => Event::TouchEnd,
            _ => return None,
        })
    }
}

impl Window {
    fn subscribe_to<T, F>(&self, handler: &Rc<F>, event_name: &str)
    where
        T: wasm_bindgen::convert::FromWasmAbi + 'static,
        T: AsRef<web_sys::Event>,
        T: Clone,
        Event: Convert<T>,
        F: Fn(Event) + 'static,
    {
        let handler = handler.clone();
        let canvas = self.canvas.clone();
        let handler = move |event: T| {
            canvas.focus().unwrap();
            if event.as_ref().type_() == "contextmenu" {
                event.as_ref().prevent_default();
            }
            if let Some(e) = Convert::convert(event.clone()) {
                handler(e);
                event.as_ref().prevent_default();
            }
        };
        let handler = wasm_bindgen::closure::Closure::wrap(Box::new(handler) as Box<dyn Fn(T)>);
        self.canvas
            .add_event_listener_with_callback(event_name, handler.as_ref().unchecked_ref())
            .unwrap();
        handler.forget(); // TODO: not forget
    }
    pub(crate) fn subscribe_events<F: Fn(Event) + 'static>(&self, handler: F) {
        let handler = Rc::new(handler);
        let handler = &handler;
        self.subscribe_to::<web_sys::KeyboardEvent, _>(handler, "keydown");
        self.subscribe_to::<web_sys::KeyboardEvent, _>(handler, "keyup");
        self.subscribe_to::<web_sys::MouseEvent, _>(handler, "mousedown");
        self.subscribe_to::<web_sys::MouseEvent, _>(handler, "mouseup");
        self.subscribe_to::<web_sys::MouseEvent, _>(handler, "mousemove");
        self.subscribe_to::<web_sys::WheelEvent, _>(handler, "wheel");
        self.subscribe_to::<web_sys::TouchEvent, _>(handler, "touchstart");
        self.subscribe_to::<web_sys::TouchEvent, _>(handler, "touchmove");
        self.subscribe_to::<web_sys::TouchEvent, _>(handler, "touchend");
        self.subscribe_to::<web_sys::TouchEvent, _>(handler, "touchcancel");
        self.subscribe_to::<web_sys::MouseEvent, _>(handler, "contextmenu");
    }
}
