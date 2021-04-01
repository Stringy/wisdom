mod scope;
mod slow;
mod builtin;
mod value;
pub mod error;

pub use slow::*;

extern crate ast;
extern crate tokenizer;

use std::path::PathBuf;

use crate::error::Error;
use common::WisdomError;

pub trait Interpreter<T, W: WisdomError> {
    ///
    /// Evaluate a single line of input and return the result
    ///
    fn eval_line(&mut self, input: &str) -> Result<T, W>;

    ///
    /// Evaluate an entire file.
    ///
    fn eval_file<P: Into<PathBuf>>(&mut self, path: P) -> Result<T, W>;
}
