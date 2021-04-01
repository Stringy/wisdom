mod stmt;
mod expr;
mod func;
mod operation;
mod error;
mod ext;

pub use stmt::*;
pub use expr::*;
pub use func::*;
pub use operation::*;

use common::{Span, Position};
use tokenizer::Token;

extern crate common;
extern crate tokenizer;

pub type NodeId = u32;

#[derive(Clone)]
pub struct Ident {
    pub position: Position,
    pub name: String,
}

impl From<&Token> for Ident {
    fn from(t: &Token) -> Self {
        Self {
            position: t.position.clone(),
            name: t.literal.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Typ {
    pub ident: Ident
}