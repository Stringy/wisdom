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
    Literal {
        kind: LiteralKind,
    },
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    SemiColon,
    LeftParen,
    RightParen,
    Lt,
    LtEq,
    Gt,
    GtEq,
    AndAnd,
    OrOr,
    EqEq,
    NotEq,
    BinOp(BinOpKind),
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum LiteralKind {
    Int {
        base: Base,
    },
    Float,
    String,
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Base {
    Hex,
    Dec,
    Bin,
    Oct,
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum BinOpKind {
    And,
    Or,
    Xor,
    Not,
    Mod,
    ShiftLeft,
    ShiftRight,
}

impl TokenKind {
    ///
    /// Returns whether this TokenKind is an arithmetic operator,
    /// otherwise returns false.
    ///
    pub fn is_operator(&self) -> bool {
        use TokenKind::*;
        match *self {
            Add | Sub | Mul | Div |
            Lt | LtEq |
            Gt | GtEq |
            AndAnd | OrOr | EqEq | NotEq |
            BinOp(..) => {
                true
            }
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_operator() {
        assert!(TokenKind::Add.is_operator());
        assert!(!TokenKind::LeftParen.is_operator());
    }
}