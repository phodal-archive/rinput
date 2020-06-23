use rustbox::{RustBox, Event};

use crate::input::Input;
use crate::keyboard::Key;

pub struct Editor {
    rb: RustBox,
}

impl Editor {
    pub fn new(source: Input, rb: RustBox) -> Editor {
        Editor {
            rb,
        }
    }

    pub fn start(&mut self) {}
}