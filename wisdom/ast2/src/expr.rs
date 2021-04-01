use common::Span;
use crate::{Ident, Block, NodeId};

pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

pub enum ExprKind {
    /// a = 10
    /// expr lhs to allow for future additions such as arrays
    /// i.e. foo[1] = 10;
    Assign(Expr, Expr),
    /// a += 10
    AssignOp(Expr, Op, Expr),
    /// `for <expr> { <block> }`
    For(Expr, Block),
    /// `while <expr> { <block> }`
    While(Expr, Block),
    /// `if <expr> { <block> } else { <block> }
    If(Expr, Block, Option<Expr>),
    /// foo(a, b)
    Call(Expr, Vec<Expr>),
}