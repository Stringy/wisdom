use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::fmt;

use common::Position;
use tokenizer::{FromTokens, Token, TokenStream};
use tokenizer::TokenKind::*;

use crate::{BinOp, Block, Ident, Value};
use crate::error::ErrorKind::UnmatchedExpr;
use crate::error::ParserError;
use crate::ext::VecDequePopTwo;

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub position: Position,
}

#[derive(Clone)]
pub enum ExprKind {
    /// a = 10
    /// expr lhs to allow for future additions such as arrays
    /// i.e. foo[1] = 10;
    Assign(Box<Expr>, Box<Expr>),
    /// a += 10
    // TODO: AssignOp(Expr, Expr),
    /// a + 5
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    /// `for <expr> { <block> }`
    // For(Expr, Block),
    /// `while <expr> { <block> }`
    While(Box<Expr>, Block),
    /// `if <expr> { <block> } else { <block> }
    If(Box<Expr>, Block, Option<Box<Expr>>),
    /// { <expr> }
    Block(Block),
    /// foo(a, b)
    Call(Box<Expr>, Vec<Box<Expr>>),
    /// A literal `1`, `"two"` etc
    Literal(Value),
    /// A named identifier (variable)
    Ident(Ident),
}

impl Debug for ExprKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::Assign(_, _) => write!(f, "ExprKind::Assign"),
            ExprKind::BinOp(_, _, _) => write!(f, "ExprKind::BinOp"),
            ExprKind::Call(_, _) => write!(f, "ExprKind::Call"),
            ExprKind::Literal(_) => write!(f, "ExprKind::Literal"),
            ExprKind::Ident(_) => write!(f, "ExprKind::Ident"),
            ExprKind::While(_, _) => write!(f, "ExprKind::While"),
            ExprKind::If(_, _, _) => write!(f, "ExprKind::If"),
            ExprKind::Block(_) => write!(f, "ExprKind::Block"),
        }
    }
}

// TODO: more error construction helpers would be very useful
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

    ///
    /// Recursively parses an expression from the token stream.
    ///
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
                    // TODO: perhaps a literal should just contain the string repr (and move Value somewhere else)
                    operands.push_back(Expr::new(ExprKind::Literal(Value::from_tokens(tokens)?), tok.position.clone()))
                }
                Identifier => {
                    match tok.literal.as_str() {
                        "while" => {
                            tokens.consume();
                            let condition = Expr::parse_expr(tokens)?;
                            let block = Block::from_tokens(tokens)?;
                            return Ok(Expr::new(ExprKind::While(condition.into(), block), tok.position.clone()));
                        }
                        "if" => {
                            tokens.consume();
                            let condition = Expr::parse_expr(tokens)?;
                            let block = Block::from_tokens(tokens)?;
                            let else_expr = if let Some(tok) = tokens.peek_ident("else") {
                                tokens.consume();
                                if let Some(_) = tokens.peek_ident("if") {
                                    Some(Box::new(Expr::parse_expr(tokens)?))
                                } else {
                                    Some(Box::new(Expr::new(ExprKind::Block(Block::from_tokens(tokens)?), tok.position.clone())))
                                }
                            } else {
                                None
                            };

                            return Ok(Expr::new(ExprKind::If(condition.into(), block, else_expr), tok.position.clone()));
                        }
                        _ => {
                            operands.push_back(
                                Expr::parse_ident(tok, tokens)?
                            )
                        }
                    }
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

    ///
    /// Parses an identifier from the `TokenStream`. `ident` is expected to be the Identifier
    /// token, and may refer to a variable name, or function call.
    ///
    fn parse_ident(ident: &Token, tokens: &TokenStream) -> Result<Self, ParserError> {
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

impl FromTokens for Expr {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        Expr::parse_expr(tokens)
    }
}