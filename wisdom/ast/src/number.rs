use std::str::FromStr;
use std::num::{ParseIntError};
use std::fmt::{self, Display, Formatter};
use tokenizer::{TokenKind, FromTokens, TokenStream, LiteralKind, Base};
use crate::error::ParserError;
use crate::error::ErrorKind::{InvalidLit, InvalidToken, UnexpectedEOL};

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Int(pub i64);

impl Int {}

impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Int {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = i64::from_str(s)?;
        Ok(Int(inner))
    }
}

impl FromTokens for Int {
    type Error = ParserError;

    fn from_tokens(iter: &TokenStream) -> Result<Self, Self::Error> {
        let tok = iter.peek().ok_or(ParserError::new(UnexpectedEOL, iter.position()))?;

        let perr = |_| {
            ParserError::new(InvalidLit, iter.position())
        };

        match tok.kind {
            TokenKind::Literal { kind: LiteralKind::Int { base } } => {
                Ok(match base {
                    Base::Hex => Self(i64::from_str_radix(tok.literal.as_str(), 16).map_err(perr)?),
                    Base::Bin => Self(i64::from_str_radix(tok.literal.as_str(), 2).map_err(perr)?),
                    Base::Dec => Self(i64::from_str_radix(tok.literal.as_str(), 10).map_err(perr)?),
                    Base::Oct => Self(i64::from_str_radix(tok.literal.as_str(), 8).map_err(perr)?),
                })
            }
            _ => Err(ParserError::new(InvalidToken(tok.kind), Some(tok.position)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number() {
        let n: Int = Int::from_str("123").unwrap();
        assert_eq!(n, Int(123));
    }
}