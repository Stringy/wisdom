use std::fmt::Debug;

use common::{Describable, Position, WisdomError};
use tokenizer::TokenKind;

#[derive(PartialOrd, PartialEq, Debug, Clone, Copy)]
pub struct ParserError {
    pub kind: ErrorKind,
    pub position: Option<Position>,
}

impl Describable for ParserError {
    fn description(&self) -> String {
        match self.kind {
            ErrorKind::InvalidToken(tok) => format!("invalid token: {:?}", tok),
            ErrorKind::InvalidLit => format!("invalid literal"),
            ErrorKind::UnexpectedEOL => format!("unexpected end-of-line"),
            ErrorKind::UnmatchedExpr => format!("unmatched expression. Probably contains too many operators, or too few operands"),
            ErrorKind::ExpectedOperator => format!("expected operator, but didn't find one"),
            ErrorKind::ExpectedIdent(ident) => format!("expected '{}'", ident),
            ErrorKind::ExpectSemiColon => format!("expected semi-colon"),
            ErrorKind::ExpectedTokens(tokens) => {
                // TODO: make ExpectedTokens description not a debug thing
                format!("expected one of {:?}", tokens)
            }
        }
    }
}

impl WisdomError for ParserError {
    fn position(&self) -> Position {
        self.position.unwrap_or_default()
    }
}

#[derive(PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum ErrorKind {
    InvalidToken(TokenKind),
    InvalidLit,
    UnexpectedEOL,
    UnmatchedExpr,
    ExpectedOperator,
    ExpectedIdent(&'static str),
    ExpectSemiColon,
    ExpectedTokens(&'static [TokenKind]),
}

impl From<TokenKind> for ErrorKind {
    fn from(t: TokenKind) -> Self {
        Self::InvalidToken(t)
    }
}

impl ParserError {
    pub fn new<K: Into<ErrorKind>>(kind: K, position: Option<Position>) -> Self {
        Self {
            kind: kind.into(),
            position,
        }
    }
}
