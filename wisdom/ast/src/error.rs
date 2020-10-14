use std::fmt::{Debug, Formatter, Display};
use std::fmt;
use tokenizer::{Token, Position, TokenKind};

#[derive(Debug)]
pub struct Error {
    repr: Repr
}

#[derive(Debug)]
pub enum Repr {
    Simple(ErrorKind),
    Custom(Box<Custom>),
}

#[derive(Debug)]
pub struct Custom {
    kind: ErrorKind,
    position: Position,
    error: Box<dyn std::error::Error + Send + Sync>,
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidToken,
    Incomplete,
    UnexpectedEOL,
}

impl From<ErrorKind> for Error {
    fn from(k: ErrorKind) -> Self {
        Self { repr: Repr::Simple(k) }
    }
}

impl Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
