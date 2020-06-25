#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![crate_name = "rinput"]
#![crate_type = "rlib"]
#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

extern crate gag;
extern crate num_traits;
extern crate termbox_sys as termbox;

pub use editor::Editor;
pub use input::Input;
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
pub mod rustbox;

