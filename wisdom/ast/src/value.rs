use tokenizer::{FromTokens, TokenStream};
use std::str::FromStr;
use std::ops::Add;
use std::borrow::Borrow;
use crate::error::Error;
use crate::error::ErrorKind::UnexpectedEOL;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Named(String),
}

impl FromTokens for Value {
    type Error = Error;

    fn from_tokens(tokens: &mut TokenStream) -> Result<Self, Self::Error> {
        let tok = tokens.peek();
        if let Some(tok) = tok {
            use tokenizer::TokenKind::*;
            match tok.kind {
                Number => Ok(Value::Int(i64::from_str(tok.literal.as_str()).unwrap())),
                Identifier => {
                    Ok(Value::Named(tok.literal.to_owned()))
                }
                _ => unimplemented!()
            }
        } else {
            Err(UnexpectedEOL.into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}