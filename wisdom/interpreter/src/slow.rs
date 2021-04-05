use std::path::PathBuf;

use ast::{BinOp, Block, Value, Function};
use ast::{Expr, ExprKind, Stmt, StmtKind};
use tokenizer::{FromTokens, TokenStream};

use crate::{builtin, Interpreter};
use crate::error::Error;
use crate::error::ErrorKind::{InvalidAssignment, NotCallable, UndefinedVar, UnexpectedArgs};
use crate::scope::Context;
use crate::value::Operations;

#[derive(Clone)]
enum VarContext<T: Clone> {
    Ret(T),
    Norm(T),
}

impl Into<VarContext<Value>> for Value {
    fn into(self) -> VarContext<Value> {
        VarContext::Norm(self)
    }
}

macro_rules! vctx {
    ($value:expr) => {
        {
            let v = $value;
            match &v {
                VarContext::Ret(_) => return Ok(v),
                VarContext::Norm(n) => n.clone(),
            }
        }
    }
}

type Result = std::result::Result<VarContext<Value>, Error>;

pub struct SlowInterpreter {
    globals: Context,
}

impl SlowInterpreter {
    pub fn new() -> Self {
        Self {
            globals: Context::new(),
        }
    }

    fn visit_stmt(&self, stmt: &Stmt) -> Result {
        match &stmt.kind {
            StmtKind::Expr(expr) => {
                self.visit_expr(expr)
            }
            StmtKind::Fn(func) => {
                self.globals.store(func.ident.name.to_owned(), Value::Func(func.clone()));
                Ok(VarContext::Norm(Value::None))
            }
        }
    }

    fn visit_expr(&self, expr: &Expr) -> Result {
        use ExprKind::*;
        match &expr.kind {
            Assign(lhs, rhs) => {
                match &lhs.kind {
                    Ident(ident) => {
                        let value = self.visit_expr(rhs)?;
                        self.globals.store(ident.name.clone(), vctx!(value));
                        Ok(VarContext::Norm(Value::None))
                    }
                    _ => {
                        Err(Error::new(InvalidAssignment))
                    }
                }
            }
            BinOp(lhs, op, rhs) => {
                self.visit_op(vctx!(self.visit_expr(&*lhs)?), *op, vctx!(self.visit_expr(&*rhs)?))
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
                Ok(VarContext::Norm(lit.clone()))
            }
            Ident(ident) => {
                let value = self.globals.lookup(&ident.name).ok_or(Error::new(UndefinedVar(ident.name.clone())))?;
                Ok(VarContext::Norm(value))
            }
            While(cond, block) => {
                self.visit_while(cond, block)
            }
            If(cond, block, maybe_else) => {
                self.visit_if(cond, block, maybe_else)
            }
            Block(block) => self.visit_block(block),
            Ret(expr) => {
                let ret = VarContext::Ret(vctx!(self.visit_expr(expr)?));
                Ok(ret)
            },
        }
    }

    fn visit_if(&self, cond: &Expr, block: &Block, maybe_else: &Option<Box<Expr>>) -> Result {
        let mut result = Value::None;
        if vctx!(self.visit_expr(cond)?).into_bool() {
            result = vctx!(self.visit_block(block)?);
        } else {
            if let Some(expr) = maybe_else {
                result = vctx!(self.visit_expr(&*expr)?);
            }
        }
        Ok(VarContext::Norm(result))
    }

    fn visit_while(&self, cond: &Expr, block: &Block) -> Result {
        while vctx!(self.visit_expr(cond)?).into_bool() {
            self.visit_block(block)?;
        }
        Ok(VarContext::Norm(Value::None))
    }

    fn visit_block(&self, block: &Block) -> Result {
        // TODO: macro this globals push/pop pattern? scope! { ... };
        self.globals.push();
        let mut result = Value::None;
        for stmt in &block.stmts {
            result = match self.visit_stmt(stmt)? {
                VarContext::Norm(v) => v,
                VarContext::Ret(v) => {
                    self.globals.pop();
                    return Ok(VarContext::Ret(v));
                }
            }
        }
        self.globals.pop();
        Ok(VarContext::Norm(result))
    }

    fn visit_function(&self, func: &Function, args: &Vec<Value>) -> Result {
        self.globals.push();
        let mut result = Value::None;
        for (i, arg) in args.iter().enumerate() {
            self.globals.store_top(
                func.args.get(i).unwrap().name.name.to_owned(),
                arg.clone(),
            );
        }
        for stmt in &func.block.stmts {
            result = match self.visit_stmt(stmt)? {
                VarContext::Norm(v) => v,
                VarContext::Ret(v) => {
                    result = v;
                    break;
                }
            }
        }
        self.globals.pop();
        Ok(VarContext::Norm(result))
    }

    fn visit_call(&self, name: &String, args: &Vec<Box<Expr>>) -> Result {
        if let Some(func) = self.globals.lookup(name) {
            if let Value::Func(func) = func {
                if func.args.len() != args.len() {
                    return Err(Error::new(UnexpectedArgs(func.args.len(), args.len())));
                }

                let mut evaled_args = Vec::new();
                for arg in args {
                    evaled_args.push(vctx!(self.visit_expr(&arg)?));
                }

                self.visit_function(&func, &evaled_args)
            } else {
                Err(Error::new(NotCallable))
            }
        } else if builtin::exists(name) {
            let mut evaled_args = Vec::new();
            for arg in args {
                evaled_args.push(vctx!(self.visit_expr(&arg)?));
            }
            Ok(VarContext::Norm(builtin::run(&name, evaled_args)?))
        } else {
            Err(Error::new(UndefinedVar(name.clone())))
        }
    }

    fn visit_op(&self, lhs: Value, op: BinOp, rhs: Value) -> Result {
        use BinOp::*;
        let result = match op {
            Add => lhs.try_add(&rhs)?,
            Sub => lhs.try_sub(&rhs)?,
            Mul => lhs.try_mul(&rhs)?,
            Div => lhs.try_div(&rhs)?,
            EqEq => lhs.is_equal(&rhs).into(),
            NotEq => Value::Bool(!lhs.is_equal(&rhs)),
            LtEq => (lhs.is_lt(&rhs) || lhs.is_equal(&rhs)).into(),
            GtEq => (lhs.is_gt(&rhs) || lhs.is_equal(&rhs)).into(),
            Lt => lhs.is_lt(&rhs).into(),
            Gt => lhs.is_gt(&rhs).into(),
            And => lhs.and(&rhs).into(),
            Or => lhs.or(&rhs).into(),
            Xor => lhs.try_xor(&rhs)?,
            BinAnd => lhs.try_bin_and(&rhs)?,
            BinOr => lhs.try_bin_or(&rhs)?,
            Eq => panic!("invalid assignment in binop")
        };
        Ok(VarContext::Norm(result))
    }
}

impl Interpreter<Value, Error> for SlowInterpreter {
    fn eval_file<P: Into<PathBuf>>(&mut self, path: P) -> std::result::Result<Value, Error> {
        let script = std::fs::read_to_string(path.into())?;
        self.eval_script(&script)
    }

    fn eval_script(&mut self, script: &str) -> std::result::Result<Value, Error> {
        let tokens = TokenStream::new(script);
        let mut result = Value::None;
        while !tokens.is_empty() {
            let stmt = Stmt::from_tokens(&tokens)?;
            result = match self.visit_stmt(&stmt)? {
                VarContext::Norm(n) => n,
                VarContext::Ret(n) => {
                    result = n;
                    break;
                }
            }
        }
        Ok(result)
    }
}
