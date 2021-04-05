use std::fmt::{Debug, Formatter};
use std::fmt;

use common::Position;
use tokenizer::{FromTokens, Token, TokenStream, TokenKind};
use tokenizer::TokenKind::*;

use crate::{BinOp, Block, Ident, Value};
use crate::error::ErrorKind::UnmatchedExpr;
use crate::error::ParserError;
use crate::ext::VecPopTwo;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expr {
    pub kind: ExprKind,
    #[serde(skip)]
    pub position: Position,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ExprKind {
    /// A local binding
    /// let a = <expr>
    Let(Ident, Option<Box<Expr>>),
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
    /// A return statement
    Ret(Box<Expr>),
}

impl Debug for ExprKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::Let(_, _) => write!(f, "ExprKind::Let"),
            ExprKind::Assign(_, _) => write!(f, "ExprKind::Assign"),
            ExprKind::BinOp(_, _, _) => write!(f, "ExprKind::BinOp"),
            ExprKind::Call(_, _) => write!(f, "ExprKind::Call"),
            ExprKind::Literal(_) => write!(f, "ExprKind::Literal"),
            ExprKind::Ident(_) => write!(f, "ExprKind::Ident"),
            ExprKind::While(_, _) => write!(f, "ExprKind::While"),
            ExprKind::If(_, _, _) => write!(f, "ExprKind::If"),
            ExprKind::Block(_) => write!(f, "ExprKind::Block"),
            ExprKind::Ret(_) => write!(f, "ExprKind::Ret"),
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
        let mut operators: Vec<BinOp> = Vec::new();
        let mut operands: Vec<Expr> = Vec::new();

        let mut peeked = tokens.peek();
        while let Some(tok) = &peeked {
            match tok.kind {
                Whitespace => {}
                LeftParen => {
                    tokens.consume();
                    operands.push(Expr::from_tokens(tokens)?);
                    expect_or_error!(tokens, RightParen)?;
                }
                Literal { .. } => {
                    // TODO: perhaps a literal should just contain the string repr (and move Value somewhere else)
                    let value = Value::from_tokens(tokens)?;
                    let value = ExprKind::Literal(value);
                    operands.push(Expr::new(value, tok.position.clone()));
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
                        "return" => {
                            tokens.consume();
                            let expr = Expr::parse_expr(tokens)?;
                            return Ok(Expr::new(ExprKind::Ret(expr.into()), tok.position.clone()));
                        }
                        "let" => {
                            tokens.consume();
                            let ident = expect_or_error!(tokens, Identifier)?;
                            let expr = if let Some(_) = tokens.expect(TokenKind::Eq) {
                                // we expect either nothing, or =
                                Some(Box::new(Expr::parse_expr(tokens)?))
                            } else {
                                None
                            };
                            return Ok(Expr::new(ExprKind::Let((&ident).into(), expr), tok.position));
                        }
                        _ => {
                            operands.push(Expr::parse_ident(tok, tokens)?)
                        }
                    }
                }
                _ if tok.kind.is_operator() => {
                    let op = BinOp::from_tokens(tokens)?;

                    if !operators.is_empty() {
                        // keep checking against stored operators until we have a higher
                        // precedence
                        while let Some(top) = operators.get(0) {
                            if top.precedence() <= op.precedence() {
                                //
                                // If the top of the stack has higher precedence
                                // or the same as the current op, then we can process a tree.
                                // We continue to do this until we hit an operator of lower precedence
                                // in the stack.
                                // TODO: this will need to be tweaked when we introduce right-associative operators
                                //
                                let (rhs, lhs) = operands.pop_two().ok_or(
                                    ParserError::new(UnmatchedExpr, Some(tok.position))
                                )?;
                                let position = lhs.position.clone();
                                let expr = ExprKind::BinOp(lhs.into(), *top, rhs.into());
                                // pop the 'top' operator cos we've just used it
                                operators.pop();
                                // push the result for next operator / unwinding later
                                operands.push(Expr::new(expr, position));
                            } else {
                                operators.push(op);
                                break;
                            }
                        }
                        // no operators left and we haven't added the current
                        // operator so add it now
                        if operators.is_empty() {
                            operators.push(op);
                        }
                    } else {
                        // no operators so push
                        operators.push(op);
                    }
                }
                _ => break,
            }

            peeked = tokens.peek();
        }

        //
        // Unwind the remaining expressions / operators in the stacks, to construct
        // the full expression. This should be balanced (i.e. num_ops = (num_expr / 2); num_expr % 2 == 0)
        // If it isn't then we've got an invalid expression.
        //
        while operators.len() > 0 {
            let op = operators.pop().ok_or(ParserError::new(UnmatchedExpr, tokens.position()))?;
            let (rhs, lhs) = operands.pop_two().ok_or(ParserError::new(UnmatchedExpr, tokens.position()))?;
            let position = lhs.position.clone();
            match op {
                BinOp::Eq => {
                    let assign = ExprKind::Assign(lhs.into(), rhs.into());
                    operands.push(Expr::new(assign, position));
                }
                _ => {
                    let binop = ExprKind::BinOp(lhs.into(), op, rhs.into());
                    operands.push(Expr::new(binop, position));
                }
            }
        }

        Ok(operands.pop().ok_or(ParserError::new(UnmatchedExpr, tokens.position()))?)
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
                let position = ident.position.clone();
                let ident = ExprKind::Ident(ident.into());
                return Ok(Expr::new(ident, position));
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