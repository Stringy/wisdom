extern crate wisdom;
extern crate clap;
extern crate rustyline;

use clap::{Arg, App};

use std::io::{Write, BufReader};

use wisdom::interpreter::{Interpreter, SlowInterpreter};
use wisdom::ast::value::Value;
use wisdom::interpreter::error::{Error};
use std::io::{self, BufRead};
use std::fs::File;
use wisdom::common::{Position, Describable, WisdomError};
use rustyline::Editor;

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

fn handle_err_with_line(desc: String, pos: Position, line: String) {
    do_write(format!("{}\n\n    ", desc).as_str());
    do_write(line.as_str());
    do_write("\n    ");
    do_write(" ".repeat(pos.column - 1).as_str());
    do_write("^");
    do_write("\n    ");
    do_write("-".repeat(pos.column - 1).as_str());
    do_write("|\n");
}


fn handle(err: Error, filename: &str) {
    let position = err.position();
    if let Ok(line) = get_line(filename, position.line - 1) {
        do_write(format!("{}:{}:{}\n", filename, position.line, position.column).as_str());
        handle_err_with_line(err.description(), position, line);
    } else {
        panic!("Error occurred when handling an error. Damn.")
    }
}

// TODO: support reading from file
fn main() {
    let mut interp = SlowInterpreter::new();
    let mut rl = Editor::<()>::new();
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
            use rustyline::error::ReadlineError::*;
            do_write("Wisdom REPL (WELP) v1.0\n");
            loop {
                let input = rl.readline(">>> ");
                match input {
                    Ok(line) => {
                        if line == "\n" || line.is_empty() {
                            continue;
                        }

                        match interp.eval_line(line.as_str()) {
                            Ok(v) => {
                                if v != Value::None {
                                    do_write(format!("{}\n", v).as_str())
                                }
                            }
                            Err(e) => {
                                do_write(format!("{}\n", e.description()).as_str())
                            }
                        }
                    }
                    Err(Interrupted) | Err(Eof) => {
                        do_write("Exiting.\n");
                        break;
                    }
                    Err(err) => {
                        do_write(format!("Input error: {}\n", err).as_str());
                        break;
                    }
                }
            }
        }
    }
}
