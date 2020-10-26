use std::collections::VecDeque;
use std::cell::RefCell;

use common::Position;
use crate::{Token, tokenize, TokenKind};
use crate::TokenKind::Whitespace;

///
/// Defines a list of tokens, generated from an input string
///
/// It provides useful functionality for interpretting that
/// list of tokens, to construct higher structures, like an AST
///
pub struct TokenStream {
    /// The tokens themselves. VecDeque, so they can be popped
    /// from the front rather than the back.
    tokens: RefCell<VecDeque<Token>>,
}

impl TokenStream {
    ///
    /// Constructs a new Tokens structure from an input string.
    ///
    pub fn new(input: &str) -> Self {
        Self {
            tokens: RefCell::new(tokenize(input, false).collect::<VecDeque<Token>>()),
        }
    }

    ///
    /// Whether there are any tokens left.
    ///
    pub fn is_empty(&self) -> bool {
        self.tokens.borrow().is_empty()
    }

    ///
    /// Returns the current position of the tokenizer, i.e. the location
    /// of the next unconsumed token.
    ///
    pub fn position(&self) -> Option<Position> {
        if let Some(tok) = self.first() {
            Some(tok.position)
        } else {
            None
        }
    }

    ///
    /// Looks ahead at the next token, without consuming it.
    ///
    pub fn first(&self) -> Option<Token> {
        let tokens = self.tokens.borrow_mut();
        tokens.get(0).cloned()
    }

    ///
    /// Looks ahead at the second next token, without consuming it.
    ///
    pub fn second(&self) -> Option<Token> {
        let tokens = self.tokens.borrow_mut();
        tokens.get(1).cloned()
    }

    ///
    /// Seeks forward in the tokens until there is a non-whitespace token.
    ///
    pub fn skip_whitespace(&self) {
        let mut peeked = self.peek();
        while let Some(tok) = &peeked {
            if tok.kind != Whitespace {
                break;
            }
            self.consume();
            peeked = self.peek();
        }
    }

    ///
    /// If the next token is of the provided TokenKind, return it, otherwise
    /// return None
    ///
    pub fn expect(&self, kind: TokenKind) -> Option<Token> {
        self.expect_any(&[kind])
    }

    ///
    /// If the next token matches any of the provided TokenKinds, return it,
    /// otherwise return None
    ///
    pub fn expect_any(&self, kinds: &[TokenKind]) -> Option<Token> {
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
    pub fn expect_fn<F: FnOnce(TokenKind) -> bool>(&self, func: F) -> Option<Token> {
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
    pub fn expect_ignore_ws(&self, kind: TokenKind) -> Option<Token> {
        self.expect_any_ignore_ws(&[kind])
    }

    ///
    /// If the next non-whitespace `Token` matches any of the provided `TokenKind`s,
    /// return it, otherwise returns `None`
    ///
    pub fn expect_any_ignore_ws(&self, kinds: &[TokenKind]) -> Option<Token> {
        self.skip_whitespace();
        self.expect_any(kinds)
    }

    ///
    /// Look ahead at the next `Token` without consuming it.
    ///
    /// ```
    /// use tokenizer::{TokenStream, Token, TokenKind, LiteralKind, Base};
    /// use common::Position;
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
    pub fn peek(&self) -> Option<Token> {
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
    pub fn consume(&self) -> Option<Token> {
        let mut tokens = self.tokens.borrow_mut();
        tokens.pop_front()
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
    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error>;
}
