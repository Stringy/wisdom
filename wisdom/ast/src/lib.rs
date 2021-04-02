extern crate common;
extern crate tokenizer;

use common::{Position};
pub use expr::*;
pub use func::*;
pub use operation::*;
pub use stmt::*;
use tokenizer::Token;
pub use value::*;

mod stmt;
mod expr;
mod func;
mod operation;
pub mod error;
mod ext;
mod value;


#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Typ {
    pub ident: Ident
}