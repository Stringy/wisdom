use crate::{Ident, Typ, Stmt, NodeId};
use common::Span;

pub struct Function {
    pub node_id: NodeId,
    pub ident: Ident,
    pub args: Vec<ArgSpec>,
    pub ret_typ: Option<Typ>,
    pub span: Span,
}

pub struct ArgSpec {
    pub ident: Ident,
    pub typ: Option<Typ>,
    pub span: Span,
}

pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}