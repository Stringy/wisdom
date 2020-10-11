use std::str::FromStr;
use std::num::{ParseIntError};
use std::fmt::{self, Display, Formatter};
use tokenizer::{TokenKind, FromTokens, TokenStream};

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Number(pub i64);

impl Number {}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Number {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = i64::from_str(s)?;
        Ok(Number(inner))
    }
}

impl FromTokens for Number {
    type Error = ();

    fn from_tokens(iter: &mut TokenStream) -> Result<Self, Self::Error> {
        let tok = iter.expect(TokenKind::Number).ok_or(())?;
        Self::from_str(tok.literal.as_str()).map_err(|_| ())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number() {
        let n: Number = Number::from_str("123").unwrap();
        assert_eq!(n, Number(123));
    }
}