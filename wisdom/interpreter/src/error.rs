use ast::error::ParserError;
use tokenizer::Token;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind
}

impl Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorKind {
    Parser(ParserError),
    UndefinedVar(String),
    Unexpected(Token),
    InvalidType,
    IOError(std::io::ErrorKind),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Parser(p) => {
                write!(f, "{}", p)
            }
            ErrorKind::UndefinedVar(n) => {
                write!(f, "Undefined variable '{}'", n)
            }
            ErrorKind::Unexpected(t) => {
                write!(f, "Unexpected token '{}' at {}", t.literal, t.position)
            }
            ErrorKind::InvalidType => {
                write!(f, "Invalid type in expression")
            }
            ErrorKind::IOError(i) => {
                write!(f, "IO Error: {:?}", i)
            }
        }
    }
}

impl From<ParserError> for Error {
    fn from(p: ParserError) -> Self {
        Self {
            kind: ErrorKind::Parser(p)
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(i: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::IOError(i.kind())
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self {
        Self {
            kind: e
        }
    }
}