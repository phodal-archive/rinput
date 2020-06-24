#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![crate_name = "rinput"]
#![crate_type = "rlib"]
#![warn(missing_docs)]

extern crate rustbox;

pub use editor::Editor;
pub use input::Input;
#[macro_use] extern crate lazy_static;

pub use modes::{StandardMode};

mod input;
mod keyboard;
mod editor;
mod buffer;
mod command;
mod view;
mod iterators;
mod modes;
mod keymap;
mod overlay;
mod textobject;
mod log;
mod utils;
