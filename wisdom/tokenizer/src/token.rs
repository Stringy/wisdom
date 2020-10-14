use std::collections::VecDeque;
use crate::cursor::tokenize;
use crate::Position;

///
/// A Token defines a distinct entity within an input
/// text, as defined by the Wisdom language.
///
/// It is simply a pair of kind, which defines what
/// the token means, and literal which contains the literal text
/// that corresponds to this token.
///
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub position: Position,
}

///
/// A TokenKind defines a type of token, and assigns the meaning
/// behind the literal text.
///
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Identifier,
    Whitespace,
    Number,
    Add,
    Sub,
    Mul,
    Div,
    Equals,
    SemiColon,
}

impl TokenKind {
    ///
    /// Returns whether this TokenKind is an arithmetic operator,
    /// otherwise returns false.
    ///
    pub fn is_operator(&self) -> bool {
        *self == TokenKind::Add ||
            *self == TokenKind::Sub ||
            *self == TokenKind::Mul ||
            *self == TokenKind::Div
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_operator() {
        assert!(TokenKind::Add.is_operator());
        assert!(!TokenKind::Number.is_operator());
    }
}