extern crate ast;
extern crate tokenizer;

use std::path::PathBuf;

use common::WisdomError;
pub use slow::*;

mod scope;
mod slow;
mod builtin;
mod value;
pub mod error;

pub trait Interpreter<T, W: WisdomError> {
    ///
    /// Evaluate a single line of input and return the result
    ///
    fn eval_line(&mut self, input: &str) -> Result<T, W> {
        self.eval_script(input)
    }

    ///
    /// Evaluate an entire file.
    ///
    fn eval_file<P: Into<PathBuf>>(&mut self, path: P) -> Result<T, W>;

    ///
    /// Evaluate a multi-line script
    ///
    fn eval_script(&mut self, script: &str) -> Result<T, W>;
}
