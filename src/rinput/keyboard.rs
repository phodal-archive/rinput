use rustbox::{RustBox, Event};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Tab,
    Enter,
    Esc,
    Backspace,
    Right,
    Left,
    Down,
    Up,
    Delete,
    Home,
    End,
    CtrlLeft,
    CtrlRight,

    Char(char),
    Ctrl(char),
}

impl Key {
    pub fn from_special_code(code: u16) -> Option<Key> {
        match code {
            _ => None,
        }
    }
}