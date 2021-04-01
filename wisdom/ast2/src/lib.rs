mod stmt;
mod expr;
mod func;

pub use stmt::*;
pub use expr::*;
pub use func::*;
use common::Span;

extern crate common;

pub type NodeId = u32;

pub struct Ident {
    pub span: Span,
    pub name: String,
}

pub struct Typ {
    pub ident: Ident
}

pub struct Item<K = ItemKind> {
    pub node_id: NodeId,
    pub kind: K,
}

pub enum ItemKind {
    Fn(Function),
    Stmt(Stmt),
    Expr(Expr),
}