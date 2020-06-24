#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![crate_name = "rinput"]
#![crate_type = "rlib"]
#![warn(missing_docs)]

extern crate rustbox;

pub use editor::Editor;
pub use input::Input;

mod input;
mod keyboard;
mod editor;
mod buffer;
mod command;
mod view;
