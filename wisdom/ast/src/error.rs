use std::fmt::{Debug, Formatter, Display};
use std::fmt;
use tokenizer::{Position, TokenKind};

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub struct ParserError {
    kind: ErrorKind,
    position: Option<Position>,
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum ErrorKind {
    InvalidToken(TokenKind),
    InvalidLit,
    UnexpectedEOL,
    UnmatchedExpr,
    ExpectedOperator,
    ExpectedIdent(&'static str),
    ExpectSemiColon,
    ExpectedTokens(Vec<TokenKind>),
}

impl From<TokenKind> for ErrorKind {
    fn from(t: TokenKind) -> Self {
        Self::InvalidToken(t)
    }
}

impl From<Vec<TokenKind>> for ErrorKind {
    fn from(ts: Vec<TokenKind>) -> Self {
        Self::ExpectedTokens(ts)
    }
}

impl ParserError {
    pub fn new<K: Into<ErrorKind>>(kind: K, position: Option<Position>) -> Self {
        Self {
            kind: kind.into(),
            position
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParserError {}
