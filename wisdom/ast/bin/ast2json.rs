extern crate ast;
extern crate tokenizer;
extern crate serde_json;

use std::error::Error;
use tokenizer::{TokenStream, FromTokens};
use ast::Stmt;

use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).expect("Must provide path to .wis file");
    let script = std::fs::read_to_string(path.clone())?;
    let tokens = TokenStream::new(&script);
    let mut stmts = Vec::new();
    while !tokens.is_empty() {
        stmts.push(Stmt::from_tokens(&tokens).unwrap());
    }
    let json_path = Path::new(&path).with_extension("json");
    println!("{:?}", json_path);
    std::fs::write(json_path, serde_json::to_string_pretty(&stmts)?)?;
    Ok(())
}