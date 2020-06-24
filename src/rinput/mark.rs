
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Mark {
    /// For keeping track of cursors.
    Cursor(usize),

    /// For using in determining some display of characters
    DisplayMark(usize),
}
