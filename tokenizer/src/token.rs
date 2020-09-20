use crate::position::Position;

#[derive(Debug, Copy, Clone)]
pub enum TokenKind {
    Invalid,
    ///
    Whitespace,
    /// [a-zA-Z_][a-zA-Z0-9_]+
    Identifier,

    /// Strings, Integers, Regex, etc.
    Literal {
        kind: LiteralKind,
    },

    /// ;
    SemiColon,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// =
    Equals,
    /// ,
    Comma,
    /// "
    DoubleQuote,
    /// '
    SingleQuote,
    /// $
    Dollar
}

#[derive(Debug, Copy, Clone)]
pub enum LiteralKind {
    Str,
    Char,
    Int {
        base: Base
    },
    Regex {}
}

#[derive(Debug, Copy, Clone)]
pub enum Base {
    /// 0x....
    Hex,
    /// 0o....
    Oct,
    /// 0b....
    Bin,
    /// anything else
    Dec,
}

impl Default for TokenKind {
    fn default() -> Self {
        TokenKind::Invalid
    }
}

///
/// A Token represents the smallest individual parts of the source
/// code that can be converted into an AST.
///
#[derive(Debug, Default, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize
}
