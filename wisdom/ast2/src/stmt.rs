use common::Span;
use crate::{Expr, Function, Ident, NodeId};

pub struct Stmt {
    pub span: Span,
    pub kind: StmtKind,
}

pub struct Decl {
    pub span: Span,
    pub ident: Ident,
    pub expr: Expr,
}

pub enum StmtKind {
    Expr(Expr),
    Fn(Function),
}

