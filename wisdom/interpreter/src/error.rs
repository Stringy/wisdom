use std::fmt::{Display, Formatter};
use std::fmt;

use ast::error::ParserError;
use common::{Describable, WisdomError, Position};
use tokenizer::Token;

#[derive(PartialEq, Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Position,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Describable for Error {
    fn description(&self) -> String {
        match &self.kind {
            ErrorKind::Parser(p) => p.description(),
            ErrorKind::UndefinedVar(name) => format!("Undefined variable '{}'", name),
            ErrorKind::Unexpected(tok) => format!("Unexpected token '{:?}'", tok.kind),
            ErrorKind::InvalidType => format!("Invalid type in expression"),
            ErrorKind::IOError(io) => format!("IO Error: {}", io),
            ErrorKind::UnexpectedArgs(exp, act) => format!("Expected {} args, got {}", exp, act),
        }
    }
}

impl WisdomError for Error {
    fn position(&self) -> Position {
        self.position
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorKind {
    Parser(ParserError),
    UndefinedVar(String),
    Unexpected(Token),
    InvalidType,
    IOError(String),
    UnexpectedArgs(usize, usize),
}

impl From<ParserError> for Error {
    fn from(p: ParserError) -> Self {
        Self {
            kind: ErrorKind::Parser(p),
            position: p.position(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(io: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::IOError(io.to_string()),
            position: Default::default(),
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            // TODO: update to use an actual position
            position: Default::default(),
        }
    }
}

