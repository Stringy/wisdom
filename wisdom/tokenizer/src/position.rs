use std::fmt::{Display, Formatter};
use std::fmt;

///
/// Position represents a location within a source file
/// or source code.
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Position {
    /// The line number. Always greater than zero.
    pub line: usize,
    /// The column offset into the line. Always greater than zero.
    pub column: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}