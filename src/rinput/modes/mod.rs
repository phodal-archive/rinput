use crate::keyboard::Key;
use crate::command::BuilderEvent;

pub use self::standard::StandardMode;
pub use self::normal::NormalMode;
pub use self::insert::InsertMode;

mod standard;
mod normal;
mod insert;

#[derive(Copy, Clone, Debug)]
pub enum ModeType {
    Normal,
    Insert,
}

/// The concept of Iota's modes are taken from Vi.
/// 
/// A modes is a mechanism for interpreting key events and converting them into
/// commands which the Editor will interpret.
pub trait Mode {
    /// Given a Key, return a Command wrapped in a BuilderEvent for the Editor to interpret
    fn handle_key_event(&mut self, key: Key) -> BuilderEvent;
}
