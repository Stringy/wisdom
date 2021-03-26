use tokenizer::{FromTokens, TokenStream};
use std::str::FromStr;
use crate::error::{ParserError};
use crate::error::ErrorKind::{UnexpectedEOL, InvalidLit};
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::func::Function;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Named(String),
    Func(Function),
    None,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(n), Int(m)) => n == m,
            (Int(n), Float(m)) => *n as f64 == *m,
            (Float(n), Int(m)) => *n == *m as f64,
            (Bool(n), Bool(m)) => n == m,
            (String(n), String(m)) => n == m,
            (Named(n), Named(m)) => n == m,
            (Func(n), Func(m)) => n.name == m.name,
            _ => false
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Value::*;
        match (self, other) {
            (Int(n), Int(m)) => n.partial_cmp(m),
            (Int(n), Float(m)) => (*n as f64).partial_cmp(m),
            (Float(n), Int(m)) => n.partial_cmp(&(*m as f64)),
            (Bool(n), Bool(m)) => n.partial_cmp(m),
            (String(n), String(m)) => n.partial_cmp(m),
            (Named(n), Named(m)) => n.partial_cmp(m),
            _ => Option::None
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(n) => write!(f, "{}", n),
            Value::String(n) => write!(f, "{}", n),
            Value::Named(n) => write!(f, "{}", n),
            Value::Func(func) => write!(f, "{}", func),
            Value::None => write!(f, "none")
        }
    }
}

impl FromTokens for Value {
    type Error = ParserError;

    fn from_tokens(tokens: &TokenStream) -> Result<Self, Self::Error> {
        let tok = tokens.peek();
        if let Some(tok) = tok {
            use tokenizer::TokenKind::*;
            use tokenizer::LiteralKind::*;
            use tokenizer::Base::*;

            let pos = Some(tok.position);
            let err = ParserError::new(InvalidLit, pos);

            match tok.kind {
                Literal { kind } => {
                    tokens.consume();
                    match kind {
                        Int { base } => {
                            Ok(match base {
                                Hex => Self::Int(i64::from_str_radix(tok.literal.as_str(), 16).map_err(|_| err)?),
                                Dec => Self::Int(i64::from_str_radix(tok.literal.as_str(), 10).map_err(|_| err)?),
                                Oct => Self::Int(i64::from_str_radix(tok.literal.as_str(), 8).map_err(|_| err)?),
                                Bin => Self::Int(i64::from_str_radix(tok.literal.as_str(), 2).map_err(|_| err)?),
                            })
                        }
                        Float => Ok(Self::Float(f64::from_str(tok.literal.as_str()).map_err(|_| err)?)),
                        String => Ok({
                            let unescaped: std::string::String = tok.literal.chars().filter(|c| *c != '\\').collect();
                            Self::String(unescaped[1..unescaped.len() - 1].to_owned())
                        })
                    }
                }
                Identifier => {
                    tokens.consume();
                    Ok(match tok.literal.as_str() {
                        "true" => Value::Bool(true),
                        "false" => Value::Bool(false),
                        "none" => Value::None,
                        _ => Value::Named(tok.literal.to_owned())
                    })
                }
                _ => unimplemented!()
            }
        } else {
            Err(ParserError::new(UnexpectedEOL, None))
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
    // TODO: add some tests for all operations
}