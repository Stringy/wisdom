extern crate ast;
extern crate tokenizer;
extern crate ron;

use std::error::Error;
use tokenizer::{TokenStream, FromTokens};
use ast::Stmt;

use std::path::Path;
use ron::ser::PrettyConfig;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).expect("Must provide path to .wis file");
    let script = std::fs::read_to_string(path.clone())?;
    let tokens = TokenStream::new(&script);
    let mut stmts = Vec::new();
    while !tokens.is_empty() {
        stmts.push(Stmt::from_tokens(&tokens).unwrap());
    }
    let pretty = PrettyConfig::new();
    println!("{}", ron::ser::to_string_pretty(&stmts, pretty)?);
    Ok(())
}