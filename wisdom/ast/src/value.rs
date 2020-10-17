use tokenizer::{FromTokens, TokenStream};
use std::str::FromStr;
use std::ops::Add;
use std::borrow::Borrow;
use crate::error::Error;
use crate::error::ErrorKind::UnexpectedEOL;
use crate::operation::Op;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
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
                    Ok(match tok.literal.as_str() {
                        "true" => Value::Bool(true),
                        "false" => Value::Bool(false),
                        _ => Value::Named(tok.literal.to_owned())
                    })
                }
                _ => unimplemented!()
            }
        } else {
            Err(UnexpectedEOL.into())
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

#[cfg(test)]
mod test {
    use super::*;
}