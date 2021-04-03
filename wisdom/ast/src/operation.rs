use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use tokenizer::{FromTokens, TokenStream};

use crate::error::ErrorKind::ExpectedOperator;
use crate::error::ParserError;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
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

impl BinOp {
    ///
    /// Defines the operator precedence. Lower value is higher-precedence.
    /// Lifted from C's precedence rules (for consistence with other C-style languages)
    ///
    pub fn precedence(self) -> usize {
        use BinOp::*;
        match self {
            Mul | Div => 3,
            Add | Sub => 4,
            Lt | LtEq | Gt | GtEq => 6,
            EqEq | NotEq => 7,
            BinAnd => 8,
            Xor => 9,
            BinOr => 10,
            And => 11,
            Or => 12,
            Eq => 14,
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Add => write!(f, "+")?,
            BinOp::Sub => write!(f, "-")?,
            BinOp::Mul => write!(f, "*")?,
            BinOp::Div => write!(f, "/")?,
            BinOp::EqEq => write!(f, "==")?,
            BinOp::NotEq => write!(f, "!=")?,
            BinOp::LtEq => write!(f, "<=")?,
            BinOp::GtEq => write!(f, ">=")?,
            BinOp::Lt => write!(f, "<")?,
            BinOp::Gt => write!(f, ">")?,
            BinOp::And => write!(f, "&&")?,
            BinOp::Or => write!(f, "||")?,
            BinOp::Xor => write!(f, "^")?,
            BinOp::BinAnd => write!(f, "&")?,
            BinOp::BinOr => write!(f, "|")?,
            BinOp::Eq => write!(f, "=")?
        };
        Ok(())
    }
}

impl FromStr for BinOp {
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
            "=" => Self::Eq,
            _ => return Err(())
        })
    }
}

impl FromTokens for BinOp {
    type Error = ParserError;

    fn from_tokens(iter: &TokenStream) -> Result<Self, Self::Error> {
        let tok = iter.expect_fn(|k| k.is_operator()).ok_or(
            ParserError::new(ExpectedOperator, iter.position())
        )?;
        Self::from_str(tok.literal.as_str()).map_err(|_| ParserError::new(ExpectedOperator, Some(tok.position)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operation() {
        assert_eq!(BinOp::from_str("+").unwrap(), BinOp::Add);
        assert_eq!(BinOp::from_str("-").unwrap(), BinOp::Sub);
        assert_eq!(BinOp::from_str("*").unwrap(), BinOp::Mul);
        assert_eq!(BinOp::from_str("/").unwrap(), BinOp::Div);
    }

    #[test]
    fn test_invalid_op() {
        assert!(BinOp::from_str("invalid").is_err())
    }
}
