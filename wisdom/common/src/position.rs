use std::fmt;
use std::fmt::Display;

///
/// A position describes a place in the source code.
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Position {
    pub line: usize,
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
