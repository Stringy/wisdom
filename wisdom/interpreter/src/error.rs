use std::fmt::{Display, Formatter};
use std::fmt;

use ast::error::ParserError;
use common::{Position, WisdomError};
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

impl Error {
    fn description(&self) -> String {
        match &self.kind {
            ErrorKind::Parser(p) => format!("{}", p),
            ErrorKind::UndefinedVar(name) => format!("Undefined variable '{}'", name),
            ErrorKind::Unexpected(tok) => format!("Unexpected token '{:?}'", tok.kind),
            ErrorKind::InvalidType => format!("Invalid type in expression"),
            ErrorKind::InvalidRegex(e) => format!("Failed to compile regex: {}", e),
            ErrorKind::IOError(io) => format!("IO Error: {}", io),
            ErrorKind::UnexpectedArgs(exp, act) => format!("Expected {} args, got {}", exp, act),
            ErrorKind::InvalidAssignment => format!("Invalid assignment"),
            ErrorKind::NotCallable => format!("not callable"),
            ErrorKind::BreakInWrongContext => format!("unable to use 'break' in this context"),
            ErrorKind::ContinueInWrongContext => format!("unable to use 'continue' in this context")
        }
    }
}

impl WisdomError for Error {
    fn position(&self) -> Position {
        self.position
    }
}

impl std::error::Error for Error {}

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorKind {
    Parser(ParserError),
    UndefinedVar(String),
    Unexpected(Token),
    InvalidType,
    InvalidRegex(regex::Error),
    InvalidAssignment,
    NotCallable,
    IOError(String),
    UnexpectedArgs(usize, usize),
    BreakInWrongContext,
    ContinueInWrongContext,
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

