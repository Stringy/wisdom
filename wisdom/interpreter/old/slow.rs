use ast::value::Value;
use tokenizer::{TokenStream, FromTokens, TokenKind};
use ast::expr::Expr;
use ast::operation::Op;
use ast::stmt::Stmt;
use ast::keywords::STMT_START;
use ast::block::Block;
use std::path::PathBuf;

use crate::scope::Context;
use crate::{Interpreter, builtin, Error};
use crate::error::ErrorKind::*;
use crate::value::Operations;
use ast::func::Function;

pub struct SlowInterpreter {
    globals: Context,
}

impl SlowInterpreter {
    pub fn new() -> Self {
        Self {
            globals: Context::new(),
        }
    }

    fn visit_stmt(&mut self, stmt: Stmt) -> Result<Value, Error> {
        use Stmt::*;
        match stmt {
            Assignment(Value::Named(name), expr) => {
                let value = self.visit_expr(expr)?;
                self.globals.store(name, value.clone());
                Ok(value)
            }
            While(expr, block) => {
                self.visit_while(expr, block)
            }
            Call(name, args) => {
                self.visit_call(name, args)
            }
            _ => unimplemented!()
        }
    }

    fn visit_call(&mut self, name: Value, args: Vec<Expr>) -> Result<Value, Error> {
        if let Value::Named(name) = name {
            if let Some(func) = self.globals.lookup(&name) {
                if let Value::Func(func) = func {
                    if func.args.len() != args.len() {
                        return Err(Error::new(UnexpectedArgs(func.args.len(), args.len())));
                    }

                    for (i, arg) in args.iter().enumerate() {
                        self.globals.store(
                            func.args.get(i).unwrap().name.to_owned(),
                            self.visit_expr(arg.to_owned())?,
                        );
                    }

                    self.globals.push();
                    let mut result = Value::None;
                    for stmt in func.body {
                        result = self.visit_stmt(stmt)?;
                    }
                    self.globals.pop();

                    Ok(result)
                } else {
                    Err(Error::new(InvalidType))
                }
            } else {
                if builtin::exists(&name) {
                    let mut evaled_args = Vec::new();
                    for arg in args {
                        evaled_args.push(self.visit_expr(arg.to_owned())?);
                    }
                    builtin::run(&name, evaled_args)
                } else {
                    Err(Error::new(UndefinedVar(name.to_owned())))
                }
            }
        } else {
            panic!("expected named value in call statement");
        }
    }

    fn visit_while(&mut self, expr: Expr, block: Block) -> Result<Value, Error> {
        while self.visit_expr(expr.to_owned())?.into_bool() {
            self.visit_block(block.to_owned())?;
        }
        Ok(Value::None)
    }

    fn visit_block(&mut self, block: Block) -> Result<Value, Error> {
        self.globals.push();
        for stmt in block.stmts {
            self.visit_stmt(stmt.to_owned())?;
        }
        self.globals.pop();
        Ok(Value::None)
    }

    fn visit_expr(&self, expr: Expr) -> Result<Value, Error> {
        match expr {
            Expr::Leaf(v) => {
                match v {
                    Value::Named(name) => {
                        let value = self.globals.lookup(&name).ok_or(Error::new(UndefinedVar(name.to_owned())))?;
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

    fn visit_op(&self, lhs: Value, op: Op, rhs: Value) -> Result<Value, Error> {
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

    fn infer(&mut self, tokens: &TokenStream) -> Result<Value, Error> {
        if let Some(tok) = tokens.first() {
            match tok.kind {
                TokenKind::Literal { .. } => {
                    let expr = Expr::from_tokens(&tokens)?;
                    self.visit_expr(expr)
                }
                TokenKind::Identifier => {
                    if STMT_START.contains(&tok.literal.as_str()) {
                        let stmt = Stmt::from_tokens(&tokens)?;
                        self.visit_stmt(stmt)
                    } else if tok.literal == "fn" {
                        let func = Function::from_tokens(&tokens)?;
                        self.globals.store(func.name.clone(), Value::Func(func));
                        Ok(Value::None)
                    } else if let Some(next) = tokens.second() {
                        match next.kind {
                            _ if next.kind.is_operator() => {
                                let expr = Expr::from_tokens(&tokens)?;
                                self.visit_expr(expr)
                            }
                            TokenKind::Eq => {
                                let assign = Stmt::from_tokens(&tokens)?;
                                self.visit_stmt(assign)
                            }
                            TokenKind::LeftParen => {
                                let call = Stmt::from_tokens(&tokens)?;
                                self.visit_stmt(call)
                            }
                            _ => Err(Error::new(Unexpected(next.to_owned())))
                        }
                    } else {
                        self.globals.lookup(&tok.literal).ok_or(Error::new(UndefinedVar(tok.literal.to_owned())))
                    }
                }
                _ => {
                    Err(Error::new(Unexpected(tok.to_owned())))
                }
            }
        } else {
            panic!("EOF?")
        }
    }
}

impl Interpreter<Value, Error> for SlowInterpreter {
    fn eval_line(&mut self, input: &str) -> Result<Value, Error> {
        let tokens = TokenStream::new(input);
        self.infer(&tokens)
    }

    fn eval_file<P: Into<PathBuf>>(&mut self, path: P) -> Result<Value, Error> {
        use std::fs;
        let script = fs::read_to_string(path.into())?;
        let tokens = TokenStream::new(script.as_str());
        while !tokens.is_empty() {
            self.infer(&tokens)?;
        }
        Ok(Value::None)
    }
}
