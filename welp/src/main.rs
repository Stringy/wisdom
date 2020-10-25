extern crate wisdom;
extern crate clap;

use clap::{Arg, App};

use std::io::Write;

use wisdom::interpreter::{Interpreter, SlowInterpreter};
use wisdom::ast::value::Value;

fn do_write(msg: &str) {
    std::io::stdout().write(msg.as_bytes()).unwrap();
    std::io::stdout().flush().unwrap();
}

fn get_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
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
                Err(_) => do_write(format!("failed to run {}\n", filename).as_str()),
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
                    },
                    Err(e) => do_write(format!("{}\n", e).as_str())
                }
            }
        }
    }
}
