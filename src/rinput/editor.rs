use rustbox::{RustBox, Event};

// use input::Input;
// use keyboard::Key;
use crate::Key;
use crate::Input;

pub struct Editor {
    rb: RustBox,
}

impl Editor {
    pub fn new(source: Input, rb: RustBox) -> Editor {
        Editor {
            rb,
        }
    }
}