mod scope;
mod slow;
mod builtin;
mod value;
pub mod error;

pub use slow::*;

extern crate ast;
extern crate tokenizer;

use std::path::PathBuf;
use ast::value::Value;

use crate::error::Error;

pub trait Interpreter {
    ///
    /// Evaluate a single line of input and return the result
    ///
    fn eval_line(&mut self, input: &str) -> Result<Value, Error>;

    ///
    /// Evaluate an entire file.
    ///
    fn eval_file<P: Into<PathBuf>>(&mut self, path: P) -> Result<Value, Error>;
}
