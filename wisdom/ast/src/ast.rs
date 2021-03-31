use tokenizer::BinOpKind;
use crate::block::Block;

pub struct Expr {}

pub enum ExprKind {
    // Name, list of args
    Call(String, Vec<Expr>),
    Binary(BinOpKind, Expr, Expr),
    If(Expr, Block, Option<Block>),
    While(Expr, Block),
    Assign(Expr, Expr),
    AssignOp(BinOpKind, Expr, Expr),
    Return(Option<Expr>),
}