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
    /// Store an optional previous token, to allow peeking
    /// ahead.
    prev: Option<Token>,
}

impl TokenStream {
    ///
    /// Constructs a new Tokens structure from an input string.
    ///
    pub fn new(input: &str) -> Self {
        Self {
            tokens: tokenize(input).collect(),
            prev: None,
        }
    }

    ///
    /// Seeks forward in the tokens until there is a non-whitespace token.
    ///
    pub fn skip_whitespace(&mut self) {
        let mut tok = self.peek();
        while tok.is_some() && tok.unwrap().kind == TokenKind::Whitespace {
            self.next();
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
            self.next()
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
    /// assert_eq!(tokens.next(), tok);
    /// ```
    ///
    pub fn peek(&mut self) -> Option<Token> {
        // if we've recently peeked, just return that
        // otherwise get the next and keep it.
        if self.prev.is_none() {
            // Strictly speaking we're consuming from the vector,
            // but we keep the token for the next call to next
            self.prev = self.tokens.pop_front();
        }
        self.prev.clone()
    }

    ///
    /// Returns and consumes the next `Token` in the list.
    ///
    /// ```
    /// use tokenizer::{Token, TokenStream};
    /// use tokenizer::TokenKind::*;
    ///
    /// let mut tokens = TokenStream::new("1+2");
    /// assert_eq!(tokens.next().unwrap().kind, Number);
    /// assert_eq!(tokens.next().unwrap().kind, Add);
    /// assert_eq!(tokens.next().unwrap().kind, Number);
    /// ```
    ///
    pub fn next(&mut self) -> Option<Token> {
        // if we've peeked, return that
        if self.prev.is_some() {
            let tok = self.prev.clone();
            // reset prev so peeking works
            self.prev = None;
            tok
        } else {
            // we haven't stored anything so return the front of the
            // list
            self.tokens.pop_front()
        }
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
