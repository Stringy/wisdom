extern crate wisdom;
extern crate clap;

use clap::{Arg, App};

use std::io::{Write, BufReader};

use wisdom::interpreter::{Interpreter, SlowInterpreter};
use wisdom::ast::value::Value;
use wisdom::interpreter::error::{Error, ErrorKind};
use std::io::{self, BufRead};
use std::fs::File;
use wisdom::ast::error::ParserError;
use wisdom::interpreter::error::ErrorKind::Parser;
use wisdom::tokenizer::Position;

fn do_write(msg: &str) {
    std::io::stdout().write(msg.as_bytes()).unwrap();
    std::io::stdout().flush().unwrap();
}

fn get_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn get_line(filename: &str, line: usize) -> io::Result<String> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().nth(line).unwrap_or(Err(io::ErrorKind::NotFound.into()))
}

fn handle_parser_error_with_line(pe: ParserError, line: String) {
    use wisdom::ast::error::ErrorKind::*;
    let pos = pe.position.unwrap_or(Position { line: 1, column: line.len() });

    match pe.kind {
        InvalidToken(kind) => do_write(format!("invalid token {:?}", kind).as_str()),
        InvalidLit => do_write("invalid literal"),
        UnexpectedEOL => do_write("unexpected EOL"),
        UnmatchedExpr => do_write("unmatched expression"),
        ExpectedOperator => do_write("expected operator"),
        ExpectedIdent(ident) => do_write(format!("expected ident '{}'", ident).as_str()),
        ExpectSemiColon => do_write("expected semi-colon"),
        ExpectedTokens(tokens) => do_write(format!("expected one of: {:?}", tokens).as_str())
    }

    do_write("\n\n\t");
    do_write(line.as_str());
    do_write("\t");
    do_write(" ".repeat(pos.column - 1).as_str());
    do_write("^");
    do_write("\n\t");
    do_write("-".repeat(pos.column - 1).as_str());
    do_write("|\n");
}

fn handle_parser_error(pe: ParserError, filename: &str) {
    if let Some(pos) = pe.position {
        if let Ok(line) = get_line(filename, pos.line - 1) {
            handle_parser_error_with_line(pe, line);
        } else {
            panic!("Error occurred when handling an error. Damn.")
        }
    }
}

fn handle(err: Error, filename: &str) {
    let kind = err.kind;
    match kind {
        ErrorKind::Parser(pe) => handle_parser_error(pe, filename),
        ErrorKind::UndefinedVar(_name) => {}
        ErrorKind::Unexpected(_token) => {}
        ErrorKind::InvalidType => {
            do_write("invalid type for operation\n");
        }
        ErrorKind::IOError(io) => {
            do_write(format!("failed to open file: {:?}", io).as_str());
        }
    }
}

// TODO: support reading from file
fn main() {
    let mut interp = SlowInterpreter::new();
    let args = App::new("WELP")
        .version("0.1")
        .author("Giles Hutton")
        .arg(
            Arg::with_name("file")
                .help("run a given wisdom file")
                .takes_value(true)
        ).get_matches();

    match args.value_of("file") {
        Some(filename) => {
            match interp.eval_file(filename) {
                Err(e) => {
                    do_write(format!("failed to run {}\n", filename).as_str());
                    handle(e, filename);
                }
                _ => {}
            }
        }
        None => {
            do_write("Wisdom REPL (WELP) v1.0\n");
            loop {
                do_write(">>> ");
                let line = get_input();

                if line == "\n" {
                    continue;
                }

                match interp.eval_line(line.as_str()) {
                    Ok(v) => {
                        if v != Value::None {
                            do_write(format!("{}\n", v).as_str())
                        }
                    }
                    Err(e) => {
                        if let Parser(pe) = e.kind {
                            handle_parser_error_with_line(pe, line);
                        } else {
                            do_write(format!("{}\n", e).as_str());
                        }
                    }
                }
            }
        }
    }
}
