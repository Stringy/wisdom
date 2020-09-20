use std::str::Chars;

use crate::token::{TokenKind, Token, LiteralKind, Base};


//
pub(crate) struct Cursor<'a> {
    size: usize,
    prev: usize,
    idx: usize,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            size: input.len(),
            prev: 0,
            idx: 0,
            chars: input.chars()
        }
    }

    ///
    /// Get the next character from the Cursor, for lookahead
    ///
    pub(crate) fn first(&self) -> char {
        self.nth(0)
    }

    ///
    /// Get the second next character from the Cursor, for lookahead
    ///
    pub(crate) fn second(&self) -> char {
        self.nth(1)
    }

    ///
    ///
    ///
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    ///
    ///
    ///
    pub(crate) fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.idx += 1;
            Some(ch)
        } else {
            None
        }
    }

    ///
    ///
    ///
    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    ///
    ///
    ///
    fn nth(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or('\0')
    }
}

impl Cursor<'_> {
    pub(crate) fn next_token(&mut self) -> Token {
        let ch = self.next().unwrap_or('\0');
        let kind = match ch {
            ch if ch.is_whitespace() => {
                self.consume_until(|c| !c.is_whitespace());
                TokenKind::Whitespace
            },
            ch if ch.is_alphanumeric() || ch == '_' => {
                // looks like the start of an identifier.
                self.consume_until(|c| !c.is_alphanumeric() && c != '_');
                TokenKind::Identifier
            },

            ch if ch.is_numeric() => {
                self.parse_numeric_lit()
            },

            '"' => {
                self.parse_string_lit()
            }

            '\'' => {
                self.next();
                let closing = self.next().unwrap_or('\0');
                if closing != '\'' {
                    TokenKind::Invalid
                } else {
                    TokenKind::Literal {kind: LiteralKind::Char}
                }
            }

            // single-character tokens
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            ';' => TokenKind::SemiColon,
            '=' => TokenKind::Equals,
            ',' => TokenKind::Comma,
            '$' => TokenKind::Dollar,

            _ => TokenKind::Invalid
        };
        let token = Token {
            kind,
            len: self.len_consumed(),
        };
        self.prev = self.idx;
        token
    }

    fn parse_numeric_lit(&mut self) -> TokenKind {
        TokenKind::Literal { kind: LiteralKind::Int { base: Base::Dec }}
    }

    fn parse_string_lit(&mut self) -> TokenKind {
        TokenKind::Literal { kind: LiteralKind::Str }
    }

    fn len_consumed(&self) -> usize {
        self.idx - self.prev
    }

    fn consume_until<F: FnOnce(char)->bool + Copy>(&mut self, func: F) {
        let mut c = self.first();
        loop {
            c = self.first();
            if func(c) {
                break;
            }
            self.next();
        }
    }
}
