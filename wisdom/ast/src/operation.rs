use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use tokenizer::{TokenKind, FromTokens, TokenStream};
use crate::error::Error;
use crate::error::ErrorKind::InvalidToken;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    EqEq,
    NotEq,
    LtEq,
    GtEq,
    Lt,
    Gt,
    And,
    Or,
    Xor,
    BinAnd,
    BinOr,
}

impl Op {
    pub fn precendence(self) -> usize {
        match self {
            Op::Add | Op::Sub => 3,
            Op::Mul | Op::Div => 4,
            Op::Lt | Op::LtEq |
            Op::Gt | Op::GtEq => 9,
            Op::EqEq | Op::NotEq => 10,
            Op::BinAnd => 11,
            Op::Xor => 12,
            Op::BinOr => 13,
            Op::And => 14,
            Op::Or => 15,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+")?,
            Op::Sub => write!(f, "-")?,
            Op::Mul => write!(f, "*")?,
            Op::Div => write!(f, "/")?,
            Op::EqEq => write!(f, "==")?,
            Op::NotEq => write!(f, "!=")?,
            Op::LtEq => write!(f, "<=")?,
            Op::GtEq => write!(f, ">=")?,
            Op::Lt => write!(f, "<")?,
            Op::Gt => write!(f, ">")?,
            Op::And => write!(f, "&&")?,
            Op::Or => write!(f, "||")?,
            Op::Xor => write!(f, "^")?,
            Op::BinAnd => write!(f, "&")?,
            Op::BinOr => write!(f, "|")?,
        };
        Ok(())
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "==" => Self::EqEq,
            "!=" => Self::NotEq,
            "<=" => Self::LtEq,
            ">=" => Self::GtEq,
            "<" => Self::Lt,
            ">" => Self::Gt,
            "&&" => Self::And,
            "||" => Self::Or,
            "^" => Self::Xor,
            "&" => Self::BinAnd,
            "|" => Self::BinOr,
            _ => return Err(())
        })
    }
}

impl FromTokens for Op {
    type Error = Error;

    fn from_tokens(iter: &mut TokenStream) -> Result<Self, Self::Error> {
        let tok = iter.expect_any(&[
            TokenKind::Add, TokenKind::Sub, TokenKind::Mul, TokenKind::Div,
        ]).ok_or(InvalidToken)?;
        Self::from_str(tok.literal.as_str()).map_err(|_| InvalidToken.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operation() {
        assert_eq!(Op::from_str("+").unwrap(), Op::Add);
        assert_eq!(Op::from_str("-").unwrap(), Op::Sub);
        assert_eq!(Op::from_str("*").unwrap(), Op::Mul);
        assert_eq!(Op::from_str("/").unwrap(), Op::Div);
    }

    #[test]
    fn test_invalid_op() {
        assert!(Op::from_str("invalid").is_err())
    }
}
