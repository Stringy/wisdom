#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum TokenKind {
    Invalid,
    Identifier,
    Whitespace,
    Number,
    Add,
    Sub,
    Mul,
    Div,
    Equals,
}

impl TokenKind {
    pub fn is_arithmetic_operator(&self) -> bool {
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
    fn test_is_arithmetic_operator() {
        assert!(TokenKind::Add.is_arithmetic_operator());
        assert!(!TokenKind::Invalid.is_arithmetic_operator());
    }
}