use crate::scope::Context;
use crate::{Interpreter, builtin};
use crate::error::{Error, ErrorKind};
use crate::value::Operations;
use std::path::PathBuf;
use ast2::{Value, BinOp, Block};
use tokenizer::{TokenStream, FromTokens, TokenKind, LiteralKind};
use ast2::{Stmt, StmtKind, Expr, ExprKind};
use ast2::error::ParserError;
use common::WisdomError;
use crate::error::ErrorKind::{UndefinedVar, InvalidAssignment, UnexpectedArgs, NotCallable};

pub struct SlowInterpreter {
    globals: Context,
}

impl SlowInterpreter {
    pub fn new() -> Self {
        Self {
            globals: Context::new(),
        }
    }

    pub fn visit_stmt(&self, stmt: &Stmt) -> Result<Value, Error> {
        match &stmt.kind {
            StmtKind::Expr(expr) => {
                self.visit_expr(expr)
            }
            StmtKind::Fn(func) => {
                self.globals.store(func.ident.name.to_owned(), Value::Func(func.clone()));
                Ok(Value::None)
            }
        }
    }

    pub fn visit_expr(&self, expr: &Expr) -> Result<Value, Error> {
        use ExprKind::*;
        match &expr.kind {
            Assign(lhs, rhs) => {
                match &lhs.kind {
                    Ident(ident) => {
                        let value = self.visit_expr(rhs)?;
                        self.globals.store(ident.name.clone(), value);
                        Ok(Value::None)
                    }
                    _ => {
                        Err(Error::new(InvalidAssignment))
                    }
                }
            }
            BinOp(lhs, op, rhs) => {
                self.visit_op(self.visit_expr(&*lhs)?, *op, self.visit_expr(&*rhs)?)
            }
            Call(name, args) => {
                let name = match &name.kind {
                    Ident(ident) => {
                        &ident.name
                    }
                    _ => unimplemented!("meta-programmed function names??")
                };
                self.visit_call(name, args)
            }
            Literal(lit) => {
                Ok(lit.clone())
            }
            Ident(ident) => {
                self.globals.lookup(&ident.name).ok_or(Error::new(UndefinedVar(ident.name.clone())))
            }
            While(cond, block) => {
                self.visit_while(cond, block)
            }
        }
    }

    fn visit_while(&self, cond: &Expr, block: &Block) -> Result<Value, Error> {
        while self.visit_expr(cond)?.into_bool() {
            self.visit_block(block)?;
        }
        Ok(Value::None)
    }

    fn visit_block(&self, block: &Block) -> Result<Value, Error> {
        // TODO: macro this globals push/pop pattern? scope! { ... };
        self.globals.push();
        let mut result = Value::None;
        for stmt in &block.stmts {
            result = self.visit_stmt(stmt)?;
        }
        self.globals.pop();
        Ok(result)
    }

    fn visit_call(&self, name: &String, args: &Vec<Box<Expr>>) -> Result<Value, Error> {
        if let Some(func) = self.globals.lookup(name) {
            if let Value::Func(func) = func {
                if func.args.len() != args.len() {
                    return Err(Error::new(UnexpectedArgs(func.args.len(), args.len())));
                }

                let mut evaled_args = Vec::new();
                for arg in args {
                    evaled_args.push(self.visit_expr(&arg)?);
                }

                self.globals.push();
                let mut result = Value::None;
                for (i, arg) in evaled_args.iter().enumerate() {
                    self.globals.store(
                        func.args.get(i).unwrap().name.name.to_owned(),
                        arg.clone(),
                    );
                }
                for stmt in func.block.stmts {
                    result = self.visit_stmt(&stmt)?;
                }
                self.globals.pop();
                Ok(result)
            } else {
                Err(Error::new(NotCallable))
            }
        } else if builtin::exists(name) {
            let mut evaled_args = Vec::new();
            for arg in args {
                evaled_args.push(self.visit_expr(&arg)?);
            }
            builtin::run(&name, evaled_args)
        } else {
            Err(Error::new(UndefinedVar(name.clone())))
        }
    }

    fn visit_op(&self, lhs: Value, op: BinOp, rhs: Value) -> Result<Value, Error> {
        use BinOp::*;
        match op {
            Add => lhs.try_add(&rhs),
            Sub => lhs.try_sub(&rhs),
            Mul => lhs.try_mul(&rhs),
            Div => lhs.try_div(&rhs),
            EqEq => Ok(lhs.is_equal(&rhs).into()),
            NotEq => Ok(Value::Bool(!lhs.is_equal(&rhs))),
            LtEq => Ok((lhs.is_lt(&rhs) || lhs.is_equal(&rhs)).into()),
            GtEq => Ok((lhs.is_gt(&rhs) || lhs.is_equal(&rhs)).into()),
            Lt => Ok(lhs.is_lt(&rhs).into()),
            Gt => Ok(lhs.is_gt(&rhs).into()),
            And => Ok(lhs.and(&rhs).into()),
            Or => Ok(lhs.or(&rhs).into()),
            Xor => lhs.try_xor(&rhs),
            BinAnd => lhs.try_bin_and(&rhs),
            BinOr => lhs.try_bin_or(&rhs),
            Eq => panic!("invalid assignment in binop")
        }
    }
}

impl Interpreter<Value, Error> for SlowInterpreter {
    fn eval_line(&mut self, input: &str) -> Result<Value, Error> {
        let tokens = TokenStream::new(input);
        let stmt = Stmt::from_tokens(&tokens)?;
        self.visit_stmt(&stmt)
    }

    fn eval_file<P: Into<PathBuf>>(&mut self, path: P) -> Result<Value, Error> {
        use std::fs;
        let script = fs::read_to_string(path.into())?;
        let tokens = TokenStream::new(script.as_str());
        while !tokens.is_empty() {
            let stmt = Stmt::from_tokens(&tokens)?;
            self.visit_stmt(&stmt);
        }
        Ok(Value::None)
    }
}
