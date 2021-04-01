use common::{Span, Position};
use crate::{Ident, Block, NodeId, BinOp};
use tokenizer::{FromTokens, TokenStream, Token};
use crate::error::ParserError;

use std::collections::VecDeque;
use crate::ext::VecDequePopTwo;

use tokenizer::TokenKind::*;
use crate::error::ErrorKind::UnmatchedExpr;

pub struct Expr {
    pub kind: ExprKind,
    pub position: Position,
}

pub enum ExprKind {
    /// a = 10
    /// expr lhs to allow for future additions such as arrays
    /// i.e. foo[1] = 10;
    Assign(Box<Expr>, Box<Expr>),
    /// a += 10
    // AssignOp(Expr, Expr),
    /// a + 5
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    /// `for <expr> { <block> }`
    // For(Expr, Block),
    /// `while <expr> { <block> }`
    // While(Expr, Block),
    /// `if <expr> { <block> } else { <block> }
    // If(Expr, Block, Option<Expr>),
    /// foo(a, b)
    Call(Box<Expr>, Vec<Box<Expr>>),
    /// A literal `1`, `"two"` etc
    /// Token is the literal token
    Literal(Token),
    /// A named identifier (variable)
    Ident(Ident),
}

macro_rules! expect_or_error {
    ($tokens:ident, $token:ident) => {
        $tokens.expect($token).ok_or(
            ParserError::new($token, $tokens.position())
        )
    }
}

impl Expr {
    pub fn new(kind: ExprKind, position: Position) -> Self {
        Self {
            kind,
            position,
        }
    }

    fn parse_expr(tokens: &TokenStream) -> Result<Self, ParserError> {
        let mut operators: VecDeque<BinOp> = VecDeque::new();
        let mut operands: VecDeque<Expr> = VecDeque::new();

        let mut peeked = tokens.peek();
        while let Some(tok) = &peeked {
            match tok.kind {
                Whitespace => {}
                LeftParen => {
                    tokens.consume();
                    operands.push_back(Expr::from_tokens(tokens)?);
                    expect_or_error!(tokens, RightParen)?;
                }
                Literal { .. } => {
                    operands.push_back(Expr::new(ExprKind::Literal(tok.clone()), tok.position.clone()))
                }
                Identifier => {
                    operands.push_back(
                        Expr::parse_ident(tok, tokens)?
                    )
                }
                _ if tok.kind.is_operator() => {
                    let op = BinOp::from_tokens(tokens)?;
                    if let Some(top) = operators.get(0) {
                        if top.precendence() < op.precendence() {
                            operators.push_back(op);
                        } else {
                            let (rhs, lhs) = operands.pop_back_two().ok_or(
                                ParserError::new(UnmatchedExpr, Some(tok.position))
                            )?;
                            let position = lhs.position.clone();
                            match op {
                                BinOp::Eq => operands.push_back(Expr::new(ExprKind::Assign(lhs.into(), rhs.into()), position)),
                                _ => operands.push_back(Expr::new(ExprKind::BinOp(lhs.into(), op, rhs.into()), position))
                            }
                        }
                    } else {
                        // first operator or no previous operators, so always
                        // push
                        operators.push_back(op);
                    }
                }
                _ => break,
            }

            peeked = tokens.peek();
        }

        while operators.len() > 0 {
            let op = operators.pop_back().ok_or(ParserError::new(UnmatchedExpr, tokens.position()))?;
            let (rhs, lhs) = operands.pop_back_two().ok_or(ParserError::new(UnmatchedExpr, tokens.position()))?;
            let position = lhs.position.clone();
            match op {
                BinOp::Eq => operands.push_back(Expr::new(ExprKind::Assign(lhs.into(), rhs.into()), position)),
                _ => operands.push_back(Expr::new(ExprKind::BinOp(lhs.into(), op, rhs.into()), position))
            }
        }

        Ok(operands.pop_back().ok_or(ParserError::new(UnmatchedExpr, tokens.position()))?)
    }

    fn parse_ident(ident: &Token, tokens: &TokenStream) -> Result<Self, ParserError> {
        match ident.literal.as_str() {
            // using a match because later it'll be useful to differentiate between
            // types of ident, which can include keywords i.e. if, while, etc
            _ => {
                // consume the ident
                tokens.consume();
                match tokens.peek() {
                    Some(Token { kind: LeftParen, .. }) => {
                        // looks like a function call
                        // consume the Lparen
                        tokens.consume();
                        let mut args = Vec::new();
                        while let None = tokens.expect(RightParen) {
                            args.push(Expr::parse_expr(tokens)?.into());
                            if let Some(Token { kind: Comma, .. }) = tokens.peek() {
                                tokens.consume();
                            }
                        }
                        // TODO: definitely need a better way of constructing these
                        return Ok(Expr::new(
                            ExprKind::Call(
                                Expr::new(ExprKind::Ident(ident.into()), ident.position.clone()).into(),
                                args,
                            ),
                            ident.position.clone(),
                        ));
                    }
                    _ => {
                        return Ok(Expr::new(ExprKind::Ident(ident.into()), ident.position.clone()));
                    }
                }
            }
        }
    }
}

impl FromTokens for Expr {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        Expr::parse_expr(tokens)
    }
}