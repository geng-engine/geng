use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Touch {
    pub id: u64,
    // TODO force
    pub position: vec2<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    MouseDown {
        position: vec2<f64>,
        button: MouseButton,
    },
    MouseUp {
        position: vec2<f64>,
        button: MouseButton,
    },
    MouseMove {
        position: vec2<f64>,
        delta: vec2<f64>,
    },
    Wheel {
        delta: f64,
    },
    TouchStart(Touch),
    TouchMove(Touch),
    TouchEnd(Touch),
    KeyDown {
        key: Key,
    },
    KeyUp {
        key: Key,
    },
    EditText(String),
    Gamepad(gilrs::Event), // TODO window should not know about it?
}

impl Event {
    pub fn translate(&self, delta: vec2<f64>) -> Self {
        let mut result = self.clone();
        use Event::*;
        match result {
            MouseDown {
                ref mut position, ..
            }
            | MouseUp {
                ref mut position, ..
            }
            | MouseMove {
                ref mut position, ..
            } => *position += delta,
            TouchStart(ref mut touch) | TouchMove(ref mut touch) | TouchEnd(ref mut touch) => {
                touch.position += delta;
            }
            _ => {}
        }
        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Escape,
    Space,
    Enter,
    Backspace,
    Tab,

    LShift,
    RShift,

    LCtrl,
    RCtrl,

    LAlt,
    RAlt,

    Left,
    Right,
    Up,
    Down,

    PageUp,
    PageDown,
    End,
    Home,
    Insert,
    Delete,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Unknown,
}
