use common::Span;
use crate::{Expr, Function};

pub struct Stmt {
    pub span: Span,
    pub kind: StmtKind,
}

pub enum StmtKind {
    // a = <expr>
    Decl(),
    Expr(Box<Expr>),
    Fn(Box<Function>),
}

