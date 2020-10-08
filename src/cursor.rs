use std::str::Chars;
use std::iter::Peekable;

use crate::tokens::{Token, TokenKind};
use std::collections::VecDeque;

//
pub struct Cursor<'a> {
    /// Total size of the input
    size: usize,
    /// End of the previous token
    prev: usize,
    /// Current index into the chars list
    idx: usize,
    /// The input chars
    chars: Chars<'a>,
    /// All chars consumed from self.chars
    /// reset after each token consumed
    consumed: Vec<char>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            size: input.len(),
            prev: 0,
            idx: 0,
            chars: input.chars(),
            consumed: Vec::new(),
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
        self.chars.as_str().is_empty()
    }

    ///
    ///
    ///
    pub fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.idx += 1;
            self.consumed.push(ch);
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
    pub fn next_token(&mut self) -> Token {
        use crate::tokens::TokenKind::*;

        self.prev = self.idx;

        let ch = self.next().unwrap_or('\0');
        let kind = match ch {
            ch if ch.is_whitespace() => {
                self.consume_until(|c| !c.is_whitespace());
                Whitespace
            }

            ch if ch.is_numeric() => {
                self.consume_until(|c| !c.is_numeric());
                Number
            }

            '+' => Add,
            '-' => Sub,
            '*' => Mul,
            '/' => Div,

            _ => panic!("invalid character: {:?}", ch)
        };

        let token = Token {
            kind,
            literal: self.consumed.iter().collect(),
        };
        self.consumed.clear();
        token
    }

    fn consume_until<F: FnOnce(char) -> bool + Copy>(&mut self, func: F) {
        let mut c = self.first();

        loop {
            if func(c) {
                break;
            }
            self.next();
            c = self.first();
        }
    }

    fn consume_while<F: FnOnce(char) -> bool + Copy>(&mut self, func: F) {
        self.consume_until(|c| !func(c))
    }

    fn len_consumed(&self) -> usize {
        self.idx - self.prev
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item=Token> + '_ {
    let mut c = Cursor::new(input);
    std::iter::from_fn(move || {
        if c.is_eof() {
            None
        } else {
            Some(c.next_token())
        }
    })
}

pub struct Tokens {
    tokens: VecDeque<Token>,
    prev: Option<Token>,
}

impl Tokens {
    pub fn new(input: &str) -> Self {
        Self {
            tokens: tokenize(input).collect(),
            prev: None,
        }
    }

    pub fn skip_whitespace(&mut self) {
        let mut tok = self.peek();
        while tok.is_some() && tok.unwrap().kind == TokenKind::Whitespace {
            self.next();
            tok = self.peek();
        }
    }

    pub fn expect(&mut self, kind: TokenKind) -> Option<Token> {
        self.expect_any(&[kind])
    }

    pub fn expect_any(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        let tok = self.peek()?;
        if kinds.contains(&tok.kind) {
            self.next()
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        if self.prev.is_none() {
            self.prev = self.tokens.pop_front();
        }
        self.prev.clone()
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.prev.is_some() {
            let tok = self.prev.clone();
            self.prev = None;
            tok
        } else {
            self.tokens.pop_front()
        }
    }
}

pub trait FromTokens: Sized {
    type Error;

    fn from_tokens(iter: &mut Tokens) -> Result<Self, Self::Error>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cursor_simple() {
        let tokens = tokenize("1 + 1").collect::<Vec<Token>>();
        let expected = vec![
            Token { kind: TokenKind::Number, literal: "1".to_string() },
            Token { kind: TokenKind::Whitespace, literal: " ".to_string() },
            Token { kind: TokenKind::Add, literal: "+".to_string() },
            Token { kind: TokenKind::Whitespace, literal: " ".to_string() },
            Token { kind: TokenKind::Number, literal: "1".to_string() },
        ];

        assert_eq!(&tokens[..], &expected[..]);
    }
}