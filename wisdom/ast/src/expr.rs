use tokenizer::{FromTokens, TokenStream, TokenKind};

use crate::operation::Op;
use crate::value::Value;
use std::collections::VecDeque;

use crate::ext::*;
use crate::error::{Error};
use crate::error::ErrorKind::InvalidToken;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Expr {
    Leaf(Value),
    Tree(Box<Expr>, Op, Box<Expr>),
}

impl Expr {
    pub fn new_tree(lhs: Expr, op: Op, rhs: Expr) -> Self {
        Expr::Tree(Box::new(lhs), op, Box::new(rhs))
    }

    pub fn new_leaf(v: Value) -> Self {
        Expr::Leaf(v)
    }
}

impl FromTokens for Expr {
    type Error = Error;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        let mut operators: VecDeque<Op> = VecDeque::new();
        let mut operands: VecDeque<Expr> = VecDeque::new();

        let mut peeked = tokens.peek();
        while let Some(tok) = peeked.clone() {
            match tok.kind {
                TokenKind::Whitespace => {
                    // ignore any whitespace, we don't care
                }
                TokenKind::LeftParen => {
                    tokens.consume();

                    // recurse to calculate the sub-expression
                    operands.push_back(Expr::from_tokens(tokens)?);
                }
                _ if tok.kind.is_operator() => {
                    let op = Op::from_tokens(tokens)?;
                    if let Some(top) = operators.get(0) {
                        if top.precendence() < op.precendence() {
                            // op has higher precedence, so push it
                            operators.push_back(op);
                        } else {
                            // construct a tree
                            let (rhs, lhs) = operands.pop_back_two().ok_or(Error::from(InvalidToken))?;
                            operands.push_back(Expr::new_tree(lhs, op, rhs));
                        }
                    } else {
                        // first operator or no previous operators, so always
                        // push
                        operators.push_back(op);
                    }
                }
                TokenKind::Literal { .. } | TokenKind::Identifier => {
                    operands.push_back(Expr::Leaf(Value::from_tokens(tokens)?));
                }
                TokenKind::SemiColon | TokenKind::RightParen => {
                    tokens.consume();
                    break;
                }
                _ => break,
            }

            peeked = tokens.peek();
        }

        while operators.len() > 0 {
            let op = operators.pop_back().ok_or(Error::from(InvalidToken))?;
            let (rhs, lhs) = operands.pop_back_two().ok_or(Error::from(InvalidToken))?;
            let tree = Expr::new_tree(lhs, op, rhs);
            operands.push_back(tree);
        }

        Ok(operands.pop_back().ok_or(Error::from(InvalidToken))?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_simple() {
        let tokens = TokenStream::new("1 + 2");
        let expr = Expr::from_tokens(&tokens).unwrap();
        let (leaf_a, leaf_b) = (Expr::new_leaf(Value::Int(1)), Expr::new_leaf(Value::Int(2)));
        let expected = Expr::new_tree(leaf_a, Op::Add, leaf_b);

        assert_eq!(expr, expected);
    }

    #[test]
    fn test_expr_multi() {
        let tokens = TokenStream::new("1 + 1 * 5");
        let expr = Expr::from_tokens(&tokens).unwrap();
        let expected = Expr::new_tree(
            Expr::new_leaf(Value::Int(1)), Op::Add, Expr::new_tree(
                Expr::new_leaf(Value::Int(1)), Op::Mul, Expr::new_leaf(Value::Int(5)),
            ),
        );

        assert_eq!(expr, expected);
    }

    #[test]
    fn test_expr_parens() {
        let tokens = TokenStream::new("(1 + 1) * 5");
        let expr = Expr::from_tokens(&tokens).unwrap();

        let expected = Expr::new_tree(
            Expr::new_tree(Expr::Leaf(Value::Int(1)), Op::Add, Expr::Leaf(Value::Int(1))),
            Op::Mul,
            Expr::Leaf(Value::Int(5)),
        );

        assert_eq!(expr, expected);
    }

    #[test]
    fn test_expr_complex() {
        let tokens = TokenStream::new("(2 * (5 + 7)) * (6 + 2)");
        let expr = Expr::from_tokens(&tokens).unwrap();
        let expected = Expr::new_tree(
            Expr::new_tree(
                Expr::Leaf(Value::Int(2)),
                Op::Mul,
                Expr::new_tree(Expr::Leaf(Value::Int(5)), Op::Add, Expr::Leaf(Value::Int(7))),
            ),
            Op::Mul,
            Expr::new_tree(
                Expr::Leaf(Value::Int(6)),
                Op::Add,
                Expr::Leaf(Value::Int(2)),
            ),
        );

        assert_eq!(expr, expected);
    }
}