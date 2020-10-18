mod traits;

use std::fs::File;

extern crate lazy_static;
extern crate wisdom;

use lazy_static::lazy_static;

use std::io::{BufReader, Write};

use wisdom::interpreter::Interpreter;

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
    let mut interp = Interpreter::new();
    do_write("Wisdom REPL (WELP) v1.0\n");
    loop {
        do_write(">>> ");
        let line = get_input();

        if line == "\n" {
            continue;
        }

        match interp.eval_line(line.as_str()) {
            Ok(v) => do_write(format!("{}\n", v).as_str()),
            Err(_) => do_write("failed\n")
        }
    }
}
