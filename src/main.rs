mod number;
mod operation;
mod expr;
mod binding;
mod traits;
mod interp;

use std::fs::File;

extern crate lazy_static;
extern crate wisdom_tokenizer;

use lazy_static::lazy_static;

use std::io::{BufReader, Write};

use crate::expr::Expr;
use crate::binding::Binding;

use wisdom_tokenizer::cursor::{Tokens, FromTokens};
use wisdom_tokenizer::tokens::TokenKind;

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

        tokens.skip_whitespace();

        if let Some(tok) = tokens.peek() {
            if tok.kind == TokenKind::Identifier && tok.literal == "let" {
                let bind = match Binding::from_tokens(&mut tokens) {
                    Ok(b) => b,
                    Err(()) => {
                        do_write("invalid variable binding\n");
                        continue;
                    }
                };
            } else {
                let expr = Expr::from_tokens(&mut tokens);
                match expr {
                    Ok(expr) => do_write(format!("{}\n", expr.execute()).as_str()),
                    Err(_) => do_write("Invalid input\n")
                }
            }
        }
    }
}
