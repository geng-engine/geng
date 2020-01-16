use crate::*;

use stdweb::traits::{IKeyboardEvent, IMouseEvent};
use stdweb::web::event as we;

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

impl Convert<we::MouseButton> for MouseButton {
    fn convert(button: we::MouseButton) -> Option<MouseButton> {
        Some(match button {
            we::MouseButton::Left => MouseButton::Left,
            we::MouseButton::Wheel => MouseButton::Middle,
            we::MouseButton::Right => MouseButton::Right,
            _ => return None,
        })
    }
}

impl Convert<we::KeyDownEvent> for Event {
    fn convert(event: we::KeyDownEvent) -> Option<Event> {
        Convert::convert(event.code()).map(|key| Event::KeyDown { key })
    }
}

impl Convert<we::KeyUpEvent> for Event {
    fn convert(event: we::KeyUpEvent) -> Option<Event> {
        Convert::convert(event.code()).map(|key| Event::KeyUp { key })
    }
}

fn convert_position<E: we::IMouseEvent>(event: E) -> Vec2<f64> {
    let canvas: stdweb::web::html_element::CanvasElement =
        stdweb::unstable::TryInto::try_into(event.target().unwrap()).unwrap();
    vec2(
        event.offset_x(),
        canvas.height() as f64 - 1.0 - event.offset_y(),
    )
}

impl Convert<we::MouseDownEvent> for Event {
    fn convert(event: we::MouseDownEvent) -> Option<Event> {
        Convert::convert(event.button()).map(|button| Event::MouseDown {
            position: convert_position(event),
            button,
        })
    }
}

impl Convert<we::MouseUpEvent> for Event {
    fn convert(event: we::MouseUpEvent) -> Option<Event> {
        Convert::convert(event.button()).map(|button| Event::MouseUp {
            position: convert_position(event),
            button,
        })
    }
}

impl Convert<we::MouseMoveEvent> for Event {
    fn convert(event: we::MouseMoveEvent) -> Option<Event> {
        Some(Event::MouseMove {
            position: convert_position(event),
        })
    }
}

fn convert_touch(touch: stdweb::web::Touch) -> TouchPoint {
    use stdweb::web::IHtmlElement;
    let canvas: stdweb::web::html_element::CanvasElement =
        stdweb::unstable::TryInto::try_into(touch.target()).unwrap();
    let rect = canvas.get_bounding_client_rect();
    let offset_x = touch.page_x() - rect.get_left();
    let offset_y = touch.page_y() - rect.get_top();
    TouchPoint {
        position: vec2(offset_x, canvas.height() as f64 - 1.0 - offset_y),
    }
}

impl Convert<we::TouchStart> for Event {
    fn convert(event: we::TouchStart) -> Option<Event> {
        use stdweb::web::event::ITouchEvent;
        Some(Event::TouchStart {
            touches: event.touches().into_iter().map(convert_touch).collect(),
        })
    }
}

impl Convert<we::TouchMove> for Event {
    fn convert(event: we::TouchMove) -> Option<Event> {
        use stdweb::web::event::ITouchEvent;
        Some(Event::TouchMove {
            touches: event.touches().into_iter().map(convert_touch).collect(),
        })
    }
}

impl Convert<we::TouchEnd> for Event {
    fn convert(event: we::TouchEnd) -> Option<Event> {
        Some(Event::TouchEnd)
    }
}

impl Convert<we::TouchCancel> for Event {
    fn convert(event: we::TouchCancel) -> Option<Event> {
        Some(Event::TouchEnd)
    }
}

impl Window {
    pub(crate) fn subscribe_events<F: Fn(Event) + 'static>(&self, handler: F) {
        use stdweb::traits::IEvent;
        use stdweb::web::{IEventTarget, IHtmlElement};
        let handler = Rc::new(handler);
        macro_rules! setup_event {
            ($canvas:expr, $handler:expr, $event:ty) => {
                let handler = handler.clone();
                let canvas_clone = $canvas.clone();
                $canvas.add_event_listener(move |event: $event| {
                    canvas_clone.focus();
                    if let Some(e) = Convert::convert(event.clone()) {
                        handler(e);
                        event.prevent_default();
                    }
                });
            };
        }
        setup_event!(self.canvas, handler, we::KeyDownEvent);
        setup_event!(self.canvas, handler, we::KeyUpEvent);
        setup_event!(self.canvas, handler, we::MouseDownEvent);
        setup_event!(self.canvas, handler, we::MouseUpEvent);
        setup_event!(self.canvas, handler, we::MouseMoveEvent);
        setup_event!(self.canvas, handler, we::TouchStart);
        setup_event!(self.canvas, handler, we::TouchMove);
        setup_event!(self.canvas, handler, we::TouchEnd);
        setup_event!(self.canvas, handler, we::TouchCancel);

        self.canvas
            .add_event_listener(move |event: we::ContextMenuEvent| {
                event.prevent_default();
            });
    }
}
