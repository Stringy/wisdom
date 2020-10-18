pub mod context;

extern crate ast;
extern crate tokenizer;

use context::Context;
use ast::value::Value;
use tokenizer::{TokenStream, FromTokens, TokenKind};
use ast::expr::Expr;
use ast::operation::Op;
use ast::assignments::{Assign, Binding};

pub struct Interpreter {
    globals: Context,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            globals: Context::new()
        }
    }

    pub fn eval_line(&mut self, input: &str) -> Result<Value, ()> {
        let mut tokens = TokenStream::new(input);
        loop {
            return if let Some(tok) = tokens.first() {
                match tok.kind {
                    TokenKind::Literal{..} => {
                        let expr = Expr::from_tokens(&mut tokens).map_err(|_| ())?;
                        self.visit_expr(expr)
                    }
                    TokenKind::Identifier => {
                        if tok.literal == "let" {
                            self.visit_bind(Binding::from_tokens(&mut tokens).map_err(|_| ())?)
                        } else if let Some(next) = tokens.second() {
                            match next.kind {
                                _ if next.kind.is_operator() => {
                                    let expr = Expr::from_tokens(&mut tokens).map_err(|_| ())?;
                                    self.visit_expr(expr)
                                }
                                TokenKind::Eq => {
                                    let assign = Assign::from_tokens(&mut tokens).map_err(|_| ())?;
                                    self.visit_assign(assign)
                                }
                                _ => Err(())
                            }
                        } else {
                            let value = self.globals.get(tok.literal.as_str()).cloned();
                            value.ok_or(())
                        }
                    }
                    _ => {
                        Err(())
                    }
                }
            } else {
                Err(())
            };
        }
    }

    fn visit_assign(&mut self, assign: Assign) -> Result<Value, ()> {
        if self.globals.contains_key(assign.name.as_str()) {
            // we have this name already, re-assign its value to a new one
            let value = self.visit_expr(assign.value)?;
            self.globals.insert(assign.name, value.clone());
            Ok(value)
        } else {
            let value = self.visit_expr(assign.value)?;
            self.globals.insert(assign.name, value.clone());
            Ok(value)
        }
    }

    fn visit_bind(&mut self, bind: Binding) -> Result<Value, ()> {
        self.visit_assign(bind.into_inner())
    }

    fn visit_expr(&self, expr: Expr) -> Result<Value, ()> {
        match expr {
            Expr::Leaf(v) => {
                match v {
                    Value::Named(name) => {
                        let value = self.globals.get(name.as_str()).ok_or(())?;
                        Ok(value.clone())
                    }
                    _ => Ok(v)
                }
            }
            Expr::Tree(lhs, op, rhs) => {
                let lhs = self.visit_expr(*lhs)?;
                let rhs = self.visit_expr(*rhs)?;
                self.visit_op(lhs, op, rhs)
            }
        }
    }

    fn visit_op(&self, lhs: Value, op: Op, rhs: Value) -> Result<Value, ()> {
        match op {
            Op::Add => lhs.try_add(&rhs),
            Op::Sub => lhs.try_sub(&rhs),
            Op::Mul => lhs.try_mul(&rhs),
            Op::Div => lhs.try_div(&rhs),
            Op::EqEq => Ok(lhs.is_equal(&rhs).into()),
            Op::NotEq => Ok(Value::Bool(!lhs.is_equal(&rhs))),
            Op::LtEq => Ok((lhs.is_lt(&rhs) || lhs.is_equal(&rhs)).into()),
            Op::GtEq => Ok((lhs.is_gt(&rhs) || lhs.is_equal(&rhs)).into()),
            Op::Lt => Ok(lhs.is_lt(&rhs).into()),
            Op::Gt => Ok(lhs.is_gt(&rhs).into()),
            Op::And => Ok(lhs.and(&rhs).into()),
            Op::Or => Ok(lhs.or(&rhs).into()),
            Op::Xor => lhs.try_xor(&rhs),
            Op::BinAnd => lhs.try_bin_and(&rhs),
            Op::BinOr => lhs.try_bin_or(&rhs)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
