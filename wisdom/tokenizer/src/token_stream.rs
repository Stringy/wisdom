use std::collections::VecDeque;
use crate::{Token, tokenize, TokenKind};

///
/// Defines a list of tokens, generated from an input string
///
/// It provides useful functionality for interpretting that
/// list of tokens, to construct higher structures, like an AST
///
pub struct TokenStream {
    /// The tokens themselves. VecDeque, so they can be popped
    /// from the front rather than the back.
    tokens: VecDeque<Token>,
    /// Store an optional first (next) token, to allow peeking
    /// ahead.
    _first: Option<Token>,
    /// Store an optional second token, to allow peeking
    /// ahead.
    _second: Option<Token>,
}

impl TokenStream {
    ///
    /// Constructs a new Tokens structure from an input string.
    ///
    pub fn new(input: &str) -> Self {
        Self {
            tokens: tokenize(input, false).collect(),
            _first: None,
            _second: None,
        }
    }

    pub fn first(&mut self) -> Option<Token> {
        if self._first.is_none() {
            self._first = self.tokens.get(0).cloned()
        }

        self._first.clone()
    }

    pub fn second(&mut self) -> Option<Token> {
        if self._second.is_none() {
            self._second = self.tokens.get(1).cloned();
        }

        self._second.clone()
    }

    ///
    /// Seeks forward in the tokens until there is a non-whitespace token.
    ///
    pub fn skip_whitespace(&mut self) {
        let mut tok = self.peek();
        while tok.is_some() && tok.unwrap().kind == TokenKind::Whitespace {
            self.consume();
            tok = self.peek();
        }
    }

    ///
    /// If the next token is of the provided TokenKind, return it, otherwise
    /// return None
    ///
    pub fn expect(&mut self, kind: TokenKind) -> Option<Token> {
        self.expect_any(&[kind])
    }

    ///
    /// If the next token matches any of the provided TokenKinds, return it,
    /// otherwise return None
    ///
    pub fn expect_any(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        let tok = self.peek()?;
        if kinds.contains(&tok.kind) {
            self.consume()
        } else {
            None
        }
    }

    pub fn expect_fn<F: FnOnce(TokenKind) -> bool>(&mut self, func: F) -> Option<Token> {
        let tok = self.peek()?;
        if func(tok.kind) {
            self.consume()
        } else {
            None
        }
    }

    ///
    /// If the next non-whitespace `Token` matches the provided `TokenKind`, return it,
    /// otherwise returns `None`
    ///
    pub fn expect_ignore_ws(&mut self, kind: TokenKind) -> Option<Token> {
        self.expect_any_ignore_ws(&[kind])
    }

    ///
    /// If the next non-whitespace `Token` matches any of the provided `TokenKind`s,
    /// return it, otherwise returns `None`
    ///
    pub fn expect_any_ignore_ws(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        self.skip_whitespace();
        self.expect_any(kinds)
    }

    ///
    /// Look ahead at the next `Token` without consuming it.
    ///
    /// ```
    /// use tokenizer::{TokenStream, Token, TokenKind, Position};
    ///
    /// let mut tokens = TokenStream::new("1+1");
    /// let tok = Some(Token {
    ///     kind: TokenKind::Number,
    ///     literal: "1".to_string(),
    ///     position: Position {
    ///         line: 1,
    ///         column: 1
    ///     }
    /// });
    ///
    /// assert_eq!(tokens.peek(), tok);
    /// // prove that peek returns the same token
    /// assert_eq!(tokens.peek(), tok);
    /// // and that next also consumes that token
    /// assert_eq!(tokens.consume(), tok);
    /// ```
    ///
    pub fn peek(&mut self) -> Option<Token> {
        self.first()
    }

    ///
    /// Returns and consumes the next `Token` in the list.
    ///
    /// ```
    /// use tokenizer::{Token, TokenStream};
    /// use tokenizer::TokenKind::*;
    ///
    /// let mut tokens = TokenStream::new("1+2");
    /// assert_eq!(tokens.consume().unwrap().kind, Number);
    /// assert_eq!(tokens.consume().unwrap().kind, Add);
    /// assert_eq!(tokens.consume().unwrap().kind, Number);
    /// ```
    ///
    pub fn consume(&mut self) -> Option<Token> {
        let tok = self.tokens.pop_front();
        self._first = None;
        self._second = None;
        tok
    }
}

pub trait FromTokens: Sized {
    /// Implementation specific error to be returned
    /// back to whatever is triggering the parsing.
    type Error;

    ///
    /// Construct Self from a list of Tokens. Intended as a way
    /// of recursively constructing AST nodes from a Tokens structure.
    ///
    fn from_tokens(tokens: &mut TokenStream) -> Result<Self, Self::Error>;
}
