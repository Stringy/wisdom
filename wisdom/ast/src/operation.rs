use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use tokenizer::{TokenKind, FromTokens, TokenStream};

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn precendence(self) -> usize {
        match self {
            Op::Add | Op::Sub => 3,
            Op::Mul | Op::Div => 4
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+")?,
            Op::Sub => write!(f, "-")?,
            Op::Mul => write!(f, "*")?,
            Op::Div => write!(f, "/")?
        };
        Ok(())
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            _ => Err(())
        }
    }
}

impl FromTokens for Op {
    type Error = ();

    fn from_tokens(iter: &mut TokenStream) -> Result<Self, Self::Error> {
        let tok = iter.expect_any(&[
            TokenKind::Add, TokenKind::Sub, TokenKind::Mul, TokenKind::Div]
        ).ok_or(())?;
        Self::from_str(tok.literal.as_str())
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
