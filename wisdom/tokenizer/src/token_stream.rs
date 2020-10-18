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
}

impl TokenStream {
    ///
    /// Constructs a new Tokens structure from an input string.
    ///
    pub fn new(input: &str) -> Self {
        Self {
            tokens: tokenize(input, false).collect(),
        }
    }

    ///
    /// Looks ahead at the next token, without consuming it.
    ///
    pub fn first(&mut self) -> Option<Token> {
        self.tokens.get(0).cloned()
    }

    ///
    /// Looks ahead at the second next token, without consuming it.
    ///
    pub fn second(&mut self) -> Option<Token> {
        self.tokens.get(1).cloned()
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

    ///
    /// More flexible expect_* function that allows the caller to provide
    /// a closure. If that closure function returns true then we consume the token
    /// and return it, otherwise returns None
    ///
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
    /// TODO: remove whitespace functions?
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
    /// use tokenizer::{TokenStream, Token, TokenKind, Position, LiteralKind, Base};
    ///
    /// let mut tokens = TokenStream::new("1+1");
    /// let tok = Some(Token {
    ///     kind: TokenKind::Literal {
    ///          kind: LiteralKind::Int { base: Base::Dec },
    ///     },
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
    /// use tokenizer::LiteralKind::Int;
    /// use tokenizer::Base::Dec;
    ///
    /// let mut tokens = TokenStream::new("1+2");
    /// assert_eq!(tokens.consume().unwrap().kind, Literal { kind: Int { base: Dec }});
    /// assert_eq!(tokens.consume().unwrap().kind, Add);
    /// assert_eq!(tokens.consume().unwrap().kind, Literal { kind: Int { base: Dec }});
    /// ```
    ///
    pub fn consume(&mut self) -> Option<Token> {
        let tok = self.tokens.pop_front();
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
