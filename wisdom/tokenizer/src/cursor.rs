use std::str::Chars;

use common::Position;

use crate::token::{Token, TokenKind, LiteralKind};
use crate::Base;

///
/// A Cursor is responsible for breaking up an input
/// string into Tokens. It emits a single token at a time
/// and can be used to construct iterators over the input.
///
pub struct Cursor<'a> {
    /// Total size of the input
    _size: usize,
    /// End of the previous token
    prev: usize,
    /// Current index into the chars list
    idx: usize,
    /// The input chars
    chars: Chars<'a>,
    /// All chars consumed from self.chars
    /// reset after each token consumed
    consumed: Vec<char>,
    /// Current position in the source code
    position: Position,
    /// Whether or not to emit Whitespace tokens
    emit_whitespace: bool,
}

impl<'a> Cursor<'a> {
    ///
    /// Constructs a new Cursor from the input string.
    ///
    pub fn new(input: &'a str, emit_whitespace: bool) -> Self {
        Self {
            _size: input.len(),
            prev: 0,
            idx: 0,
            chars: input.chars(),
            consumed: Vec::new(),
            position: Default::default(),
            emit_whitespace,
        }
    }

    ///
    /// Get the next character from the Cursor, for lookahead
    ///
    pub fn first(&self) -> char {
        self.nth(0)
    }

    ///
    /// Get the second next character from the Cursor, for lookahead
    ///
    pub fn second(&self) -> char {
        self.nth(1)
    }

    ///
    ///
    ///
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty() || (!self.emit_whitespace && self.chars.clone().all(|c| c.is_whitespace()))
    }

    ///
    /// Consume and return the next character in the string.
    /// If there are no more characters, this function will return
    /// None
    ///
    pub fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.idx += 1;
            self.consumed.push(ch);
            self.position.column += 1;
            if ch == '\n' {
                self.position.line += 1;
                self.position.column = 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    ///
    /// Gets a copy of the remaining characters in this Cursor.
    ///
    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    ///
    /// Gets the nth item in the iterator, without consuming up to
    /// that point, as you'd normally expect. Instead it will return
    /// a copy of the nth character or a NUL byte if n is too large.
    ///
    /// TODO: refactor this potentially enormous and frequent copy.
    ///
    fn nth(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or('\0')
    }
}

impl<'a> Cursor<'a> {
    ///
    /// Advances the Cursor until it has consumed a complete Token
    /// and returns.
    ///
    /// ```
    /// use tokenizer::Cursor;
    /// use tokenizer::TokenKind;
    ///
    /// let mut cursor = Cursor::new("ident", false);
    /// let tok = cursor.next_token();
    /// assert_eq!(tok.kind, TokenKind::Identifier);
    /// assert_eq!(tok.literal.as_str(), "ident");
    /// ```
    ///
    pub fn next_token(&mut self) -> Token {
        use crate::token::TokenKind::*;
        use crate::token::BinOpKind::*;

        self.prev = self.idx;
        let saved_position = self.position.clone();

        if !self.emit_whitespace {
            self.consume_while(|c| c.is_whitespace());
            self.consumed.clear();
        }

        let ch = self.next().unwrap_or('\0');
        let kind = match ch {
            ch if ch.is_whitespace() => {
                // this won't be taken if we've consumed whitespace
                // above, so this branch is only taken if self.emit_whitespace is false
                self.consume_until(|c| !c.is_whitespace());
                Whitespace
            }

            ch if ch.is_numeric() => {
                self.consume_number_literal()
            }

            ch if self.is_ident_start(ch) => {
                self.consume_until(|c| !c.is_alphanumeric());
                Identifier
            }

            '"' => self.consume_string_literal(),

            '>' => self.expect_equals(GtEq, Gt),
            '<' => self.expect_equals(LtEq, Lt),

            // Handle all the single-character tokens
            '+' => Add,
            '-' => Sub,
            '*' => Mul,
            '/' => Div,

            '=' => self.expect_equals(EqEq, Eq),
            '|' => self.expect_next('|', OrOr, BinOp(Or)),
            '&' => self.expect_next('&', AndAnd, BinOp(And)),
            '^' => BinOp(Xor),
            '!' => self.expect_equals(NotEq, BinOp(Not)),
            '%' => BinOp(Mod),

            ';' => SemiColon,
            ':' => Colon,
            ',' => Comma,
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,

            _ => panic!("invalid character: {:?}", ch)
        };

        let token = Token {
            kind,
            literal: self.consumed.iter().collect(),
            position: saved_position,
        };
        self.consumed.clear();
        token
    }

    fn consume_string_literal(&mut self) -> TokenKind {
        let mut escaped = false;
        let mut c = self.first();
        loop {
            if match c {
                '"' => {
                    if escaped {
                        escaped = false;
                        false
                    } else {
                        true
                    }
                }
                '\\' => {
                    escaped = true;
                    false
                }
                _ => false
            } {
                break;
            }
            self.next();
            c = self.first();
        }
        self.next();
        TokenKind::Literal { kind: LiteralKind::String }
    }

    ///
    /// Consumes a numeric literal, including parsing of different bases and kinds
    /// Current base support is Hex, Dec, Bin, and Oct literals.
    /// Current kind support is Float or Int
    ///
    /// For a given literal, only the parsable part of the literal is returned in the token
    /// i.e 0x12345 => 12345
    ///     0b11100 => 11100
    /// This makes it much easier to convert into an actual value later on
    ///
    /// TODO: add some tests for consume_number_literal
    ///
    fn consume_number_literal(&mut self) -> TokenKind {
        match self.first() {
            'x' => {
                self.next();
                self.consumed.clear();
                self.consume_while(|c| c.is_ascii_hexdigit());
                TokenKind::Literal { kind: LiteralKind::Int { base: Base::Hex } }
            }
            'b' => {
                self.next();
                self.consumed.clear();
                self.consume_while(|c| c == '0' || c == '1');
                TokenKind::Literal { kind: LiteralKind::Int { base: Base::Bin } }
            }
            'o' => {
                self.next();
                self.consumed.clear();
                self.consume_while(|c| c > '0' && c < '8');
                TokenKind::Literal { kind: LiteralKind::Int { base: Base::Oct } }
            }
            _ => {
                self.consume_while(|c| c.is_numeric());
                if self.first() == '.' {
                    self.next().unwrap(); // this is safe
                    self.consume_while(|c| c.is_numeric());
                    TokenKind::Literal { kind: LiteralKind::Float }
                } else {
                    TokenKind::Literal { kind: LiteralKind::Int { base: Base::Dec } }
                }
            }
        }
    }

    ///
    /// Helper wrapper function for those two-character tokens that expect an equals
    /// i.e ==, <=, >= etc
    ///
    fn expect_equals(&mut self, is_expected: TokenKind, is_unexpected: TokenKind) -> TokenKind {
        self.expect_next('=', is_expected, is_unexpected)
    }

    ///
    /// Helper function for returning different token kinds based on the next character.
    /// If the next character is as expected, one token kind is returned, if it is anything else,
    /// another token kind is returned.
    ///
    /// If the character is as expected, we also advance to the next character.
    ///
    fn expect_next(&mut self, expected: char, is_expected: TokenKind, is_unexpected: TokenKind) -> TokenKind {
        match self.first() {
            c if c == expected => {
                self.next();
                is_expected
            }
            _ => is_unexpected
        }
    }

    ///
    /// Checks whether the given character is a valid start to an identifier
    /// i.e. is alphabetic or is an underscore.
    ///
    /// Numeric characters are invalid for starting an identifier but
    /// are allowed within the body of the identifier.
    ///
    fn is_ident_start(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    ///
    /// Consumes from the Cursor until the function provided returns true.
    /// This is useful for seeking forward to the end of a token.
    ///
    /// The Cursor will end up in a state where func(c) == true where
    /// c == self.first()
    ///
    fn consume_until<F: FnOnce(char) -> bool + Copy>(&mut self, func: F) {
        let mut c = self.first();

        loop {
            if func(c) {
                break;
            }

            // consume the character
            self.next();
            // peek at the the next one
            c = self.first();
        }
    }

    ///
    /// The opposite behaviour to consume_until, this function will consume until
    /// the given function returns false
    ///
    fn consume_while<F: FnOnce(char) -> bool + Copy>(&mut self, func: F) {
        // TODO: is there a better way of wrapping the closure like this?
        self.consume_until(|c| !func(c))
    }

    ///
    /// Returns the number of characters consumed since we last emitted
    /// a Token.
    ///
    #[allow(dead_code)]
    fn len_consumed(&self) -> usize {
        self.idx - self.prev
    }
}

///
/// Creates a Token iterator from the input string.
///
/// ```
/// use tokenizer::tokenize;
/// assert_eq!(3, tokenize("1+2", false).collect::<Vec<_>>().len())
/// ```
///
pub fn tokenize(input: &str, with_whitespace: bool) -> impl Iterator<Item=Token> + '_ {
    let mut c = Cursor::new(input, with_whitespace);
    std::iter::from_fn(move || {
        if c.is_eof() {
            None
        } else {
            Some(c.next_token())
        }
    })
}


#[cfg(test)]
mod test {
    use super::*;

    fn pos(line: usize, col: usize) -> Position {
        Position {
            line: line,
            column: col,
        }
    }

    #[test]
    fn test_cursor_simple() {
        let tokens = tokenize("1 + 1", true).collect::<Vec<Token>>();
        let expected = vec![
            Token { kind: TokenKind::Literal { kind: LiteralKind::Int { base: Base::Dec } }, literal: "1".to_string(), position: pos(1, 1) },
            Token { kind: TokenKind::Whitespace, literal: " ".to_string(), position: pos(1, 2) },
            Token { kind: TokenKind::Add, literal: "+".to_string(), position: pos(1, 3) },
            Token { kind: TokenKind::Whitespace, literal: " ".to_string(), position: pos(1, 4) },
            Token { kind: TokenKind::Literal { kind: LiteralKind::Int { base: Base::Dec } }, literal: "1".to_string(), position: pos(1, 5) },
        ];

        assert_eq!(&tokens[..], &expected[..]);
    }

    #[test]
    fn test_cursor_ident() {
        let tokens: Vec<Token> = tokenize("identifier", false).collect();
        let expected = vec![
            Token { kind: TokenKind::Identifier, literal: "identifier".to_string(), position: pos(1, 1) }
        ];

        assert_eq!(&tokens[..], &expected[..]);
    }

    #[test]
    fn test_number_hex() {
        let tokens = tokenize("0x1a3", false).collect::<Vec<Token>>();
        let expected = vec![
            Token { kind: TokenKind::Literal { kind: LiteralKind::Int { base: Base::Hex } }, literal: "1a3".to_string(), position: pos(1, 1) }
        ];
        assert_eq!(tokens, expected);
    }
}