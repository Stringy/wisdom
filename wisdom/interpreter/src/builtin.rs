use ast::Value;

use crate::error::Error;

pub fn exists(name: &String) -> bool {
    match name.as_str() {
        "print" => true,
        _ => false
    }
}

pub fn run(name: &String, args: Vec<Value>) -> Result<Value, Error> {
    match name.as_str() {
        "print" => print(args),
        _ => panic!("no such builtin function: {}", name)
    }
}

pub fn print(args: Vec<Value>) -> Result<Value, Error> {
    for arg in args {
        print!("{}", arg);
    }
    print!("\n");
    Ok(Value::None)
}
