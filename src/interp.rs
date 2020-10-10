use std::path::PathBuf;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval_line(_line: &str) -> Result<(), ()> { Ok(()) }

    pub fn eval_file<P: Into<PathBuf>>(_file: P) -> Result<(), ()> { Ok(()) }
}