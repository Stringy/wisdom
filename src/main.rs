mod number;
mod operation;
mod ast;
mod cursor;
mod tokens;

use std::fs::File;

use std::io::{BufReader, Write};
use crate::cursor::{Tokens, FromTokens};
use crate::ast::Expr;

fn do_write(msg: &str) {
    std::io::stdout().write(msg.as_bytes()).unwrap();
    std::io::stdout().flush().unwrap();
}

fn get_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    loop {
        do_write(">>> ");
        let line = get_input();
        let mut tokens = Tokens::new(line.as_str());
        let expr = Expr::from_tokens(&mut tokens);
        match expr {
            Ok(expr) => do_write(format!("{}\n", expr.execute()).as_str()),
            Err(_) => do_write("Invalid input\n")
        }
    }
}
