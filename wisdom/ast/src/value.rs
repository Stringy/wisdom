use tokenizer::{FromTokens, TokenStream};
use std::str::FromStr;
use std::ops::Add;
use std::borrow::Borrow;
use crate::error::{Error, ErrorKind};
use crate::error::ErrorKind::UnexpectedEOL;
use crate::operation::Op;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Named(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(n) => write!(f, "{}", n),
            Value::String(n) => write!(f, "{}", n),
            Value::Named(n) => write!(f, "{}", n),
        }
    }
}

impl FromTokens for Value {
    type Error = Error;

    fn from_tokens(tokens: &mut TokenStream) -> Result<Self, Self::Error> {
        let tok = tokens.peek();
        if let Some(tok) = tok {
            use tokenizer::TokenKind::*;
            use tokenizer::LiteralKind::*;
            use tokenizer::Base::*;

            match tok.kind {
                Literal { kind } => {
                    tokens.consume();
                    match kind {
                        Int { base } => {
                            Ok(match base {
                                Hex => Self::Int(i64::from_str_radix(tok.literal.as_str(), 16).map_err(|_| Error::from(ErrorKind::InvalidLit))?),
                                Bin => Self::Int(i64::from_str_radix(tok.literal.as_str(), 2).map_err(|_| Error::from(ErrorKind::InvalidLit))?),
                                Dec => Self::Int(i64::from_str_radix(tok.literal.as_str(), 10).map_err(|_| Error::from(ErrorKind::InvalidLit))?),
                                Oct => Self::Int(i64::from_str_radix(tok.literal.as_str(), 8).map_err(|_| Error::from(ErrorKind::InvalidLit))?),
                            })
                        }
                        Float => Ok(Self::Float(f64::from_str(tok.literal.as_str()).map_err(|_| Error::from(ErrorKind::InvalidLit))?)),
                        String => Ok(Self::String(tok.literal.to_owned())),
                    }
                }
                Identifier => {
                    tokens.consume();
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

impl Value {
    pub fn try_add(&self, rhs: &Value) -> Result<Value, ()> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n + m)),
                    Value::Float(m) => Ok(Value::Float(*n as f64 + m)),
                    _ => Err(())
                }
            }
            Value::Float(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Float(n + *m as f64)),
                    Value::Float(m) => Ok(Value::Float(n + m)),
                    _ => Err(())
                }
            }
            _ => Err(())
        }
    }

    pub fn try_sub(&self, rhs: &Value) -> Result<Value, ()> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n - m)),
                    Value::Float(m) => Ok(Value::Float(*n as f64 - m)),
                    _ => Err(())
                }
            }
            Value::Float(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Float(n - *m as f64)),
                    Value::Float(m) => Ok(Value::Float(n - m)),
                    _ => Err(())
                }
            }
            _ => Err(())
        }
    }

    pub fn try_mul(&self, rhs: &Value) -> Result<Value, ()> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n * m)),
                    Value::Float(m) => Ok(Value::Float(*n as f64 * m)),
                    _ => Err(())
                }
            }
            Value::Float(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Float(n * *m as f64)),
                    Value::Float(m) => Ok(Value::Float(n * m)),
                    _ => Err(())
                }
            }
            _ => Err(())
        }
    }

    pub fn try_div(&self, rhs: &Value) -> Result<Value, ()> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Float(*n as f64 / *m as f64)),
                    Value::Float(m) => Ok(Value::Float(*n as f64 / m)),
                    _ => Err(())
                }
            }
            Value::Float(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Float(n / *m as f64)),
                    Value::Float(m) => Ok(Value::Float(n / m)),
                    _ => Err(())
                }
            }
            _ => Err(())
        }
    }

    pub fn is_equal(&self, rhs: &Value) -> bool {
        *self == *rhs
    }

    pub fn is_lt(&self, rhs: &Value) -> bool {
        *self < *rhs
    }

    pub fn is_gt(&self, rhs: &Value) -> bool {
        *self > *rhs
    }

    pub fn and(&self, rhs: &Value) -> bool {
        self.into_bool() && rhs.into_bool()
    }

    pub fn or(&self, rhs: &Value) -> bool {
        self.into_bool() || rhs.into_bool()
    }

    pub fn try_xor(&self, rhs: &Value) -> Result<Value, ()> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n ^ m)),
                    _ => Err(())
                }
            }
            _ => Err(())
        }
    }

    pub fn try_bin_and(&self, rhs: &Value) -> Result<Value, ()> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n & m)),
                    _ => Err(())
                }
            }
            _ => Err(())
        }
    }

    pub fn try_bin_or(&self, rhs: &Value) -> Result<Value, ()> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n | m)),
                    _ => Err(())
                }
            }
            _ => Err(())
        }
    }

    fn into_bool(&self) -> bool {
        match self {
            Value::Int(n) => *n != 0,
            Value::Float(n) => *n != 0f64,
            Value::String(s) => !s.is_empty(),
            Value::Bool(b) => *b,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}