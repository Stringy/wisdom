use crate::Position;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}