use tokenizer::token::{FromTokens, Tokens};

use crate::number::Number;
use crate::operation::Op;

pub struct Expr {
    lhs: Number,
    rhs: Number,
    op: Op,
}

impl Expr {
    pub fn new(lhs: Number, rhs: Number, op: Op) -> Self {
        Self {
            lhs,
            rhs,
            op,
        }
    }
}

impl FromTokens for Expr {
    type Error = ();

    fn from_tokens(iter: &mut Tokens) -> Result<Self, Self::Error> {
        let lhs = Number::from_tokens(iter)?;
        iter.skip_whitespace();
        let op = Op::from_tokens(iter)?;
        iter.skip_whitespace();
        let rhs = Number::from_tokens(iter)?;

        Ok(Self {
            lhs,
            op,
            rhs,
        })
    }
}

impl Expr {
    pub fn execute(&self) -> Number {
        let result = match self.op {
            Op::Add => self.lhs.0 + self.rhs.0,
            Op::Sub => self.lhs.0 - self.rhs.0,
            Op::Mul => self.lhs.0 * self.rhs.0,
            Op::Div => self.lhs.0 / self.rhs.0
        };
        Number(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr_from_cursor() {
        let mut tokens = Tokens::new("1 + 1");
        let expr = Expr::from_tokens(&mut tokens).unwrap();
    }

    #[test]
    fn test_execute_expr_add() {
        let mut tokens = Tokens::new("1 + 1");
        let expr = Expr::from_tokens(&mut tokens).unwrap();
        assert_eq!(Number(2), expr.execute())
    }

    #[test]
    fn test_execute_expr_sub() {
        let mut tokens = Tokens::new("1 - 1");
        let expr = Expr::from_tokens(&mut tokens).unwrap();
        assert_eq!(Number(0), expr.execute())
    }

    #[test]
    fn test_execute_expr_mul() {
        let mut tokens = Tokens::new("2 * 2");
        let expr = Expr::from_tokens(&mut tokens).unwrap();
        assert_eq!(Number(4), expr.execute())
    }

    #[test]
    fn test_execute_expr_div() {
        let mut tokens = Tokens::new("2 / 2");
        let expr = Expr::from_tokens(&mut tokens).unwrap();
        assert_eq!(Number(1), expr.execute())
    }
}