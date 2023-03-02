use super::*;

#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod impl_web;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native.rs"]
mod _impl;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct TouchPoint {
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
    TouchStart {
        touches: Vec<TouchPoint>,
    },
    TouchMove {
        touches: Vec<TouchPoint>,
    },
    TouchEnd {
        touches: Vec<TouchPoint>,
    },
    KeyDown {
        key: Key,
    },
    KeyUp {
        key: Key,
    },
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
            TouchStart { ref mut touches } | TouchMove { ref mut touches } => {
                for touch in touches {
                    touch.position += delta;
                }
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

    LBracket,
    RBracket,

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

    Comma,
    Period,
    Apostrophe,
    Semicolon,
    Slash,
    Grave,
    Minus,
    Equals,

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
