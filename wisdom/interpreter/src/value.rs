use ast2::Value;
use crate::error::Error;
use crate::error::ErrorKind::InvalidType;

pub trait Operations {
    fn try_add(&self, rhs: &Value) -> Result<Value, Error>;
    fn try_sub(&self, rhs: &Value) -> Result<Value, Error>;
    fn try_mul(&self, rhs: &Value) -> Result<Value, Error>;
    fn try_div(&self, rhs: &Value) -> Result<Value, Error>;
    fn is_equal(&self, rhs: &Value) -> bool;
    fn is_lt(&self, rhs: &Value) -> bool;
    fn is_gt(&self, rhs: &Value) -> bool;
    fn and(&self, rhs: &Value) -> bool;
    fn or(&self, rhs: &Value) -> bool;
    fn try_xor(&self, rhs: &Value) -> Result<Value, Error>;
    fn try_bin_and(&self, rhs: &Value) -> Result<Value, Error>;
    fn try_bin_or(&self, rhs: &Value) -> Result<Value, Error>;
    fn into_bool(&self) -> bool;
}

impl Operations for Value {
    ///
    /// Attempts to add two values together.
    ///
    fn try_add(&self, rhs: &Value) -> Result<Value, Error> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n + m)),
                    Value::Float(m) => Ok(Value::Float(*n as f64 + m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            Value::Float(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Float(n + *m as f64)),
                    Value::Float(m) => Ok(Value::Float(n + m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            Value::String(n) => {
                match rhs {
                    Value::String(m) => Ok(Value::String((n.to_owned() + m).to_owned())),
                    _ => Err(Error::new(InvalidType))
                }
            }
            _ => Err(Error::new(InvalidType))
        }
    }

    fn try_sub(&self, rhs: &Value) -> Result<Value, Error> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n - m)),
                    Value::Float(m) => Ok(Value::Float(*n as f64 - m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            Value::Float(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Float(n - *m as f64)),
                    Value::Float(m) => Ok(Value::Float(n - m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            _ => Err(Error::new(InvalidType))
        }
    }

    fn try_mul(&self, rhs: &Value) -> Result<Value, Error> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n * m)),
                    Value::Float(m) => Ok(Value::Float(*n as f64 * m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            Value::Float(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Float(n * *m as f64)),
                    Value::Float(m) => Ok(Value::Float(n * m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            _ => Err(Error::new(InvalidType))
        }
    }

    fn try_div(&self, rhs: &Value) -> Result<Value, Error> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Float(*n as f64 / *m as f64)),
                    Value::Float(m) => Ok(Value::Float(*n as f64 / m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            Value::Float(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Float(n / *m as f64)),
                    Value::Float(m) => Ok(Value::Float(n / m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            _ => Err(Error::new(InvalidType))
        }
    }

    fn is_equal(&self, rhs: &Value) -> bool {
        *self == *rhs
    }

    fn is_lt(&self, rhs: &Value) -> bool {
        *self < *rhs
    }

    fn is_gt(&self, rhs: &Value) -> bool {
        *self > *rhs
    }

    fn and(&self, rhs: &Value) -> bool {
        self.into_bool() && rhs.into_bool()
    }

    fn or(&self, rhs: &Value) -> bool {
        self.into_bool() || rhs.into_bool()
    }

    fn try_xor(&self, rhs: &Value) -> Result<Value, Error> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n ^ m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            _ => Err(Error::new(InvalidType))
        }
    }

    fn try_bin_and(&self, rhs: &Value) -> Result<Value, Error> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n & m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            _ => Err(Error::new(InvalidType))
        }
    }

    fn try_bin_or(&self, rhs: &Value) -> Result<Value, Error> {
        match self {
            Value::Int(n) => {
                match rhs {
                    Value::Int(m) => Ok(Value::Int(n | m)),
                    _ => Err(Error::new(InvalidType))
                }
            }
            _ => Err(Error::new(InvalidType))
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

