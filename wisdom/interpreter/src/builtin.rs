use ast::{Value, Stmt};

use crate::error::{Error, ErrorKind};
use tokenizer::{TokenStream, FromTokens};

pub fn exists(name: &String) -> bool {
    match name.as_str() {
        "print" => true,
        "ast" => true,
        _ => false
    }
}

pub fn run(name: &String, args: Vec<Value>) -> Result<Value, Error> {
    match name.as_str() {
        "print" => print(args),
        "ast" => ast(args),
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

pub fn ast(args: Vec<Value>) -> Result<Value, Error> {
    use ron::ser::PrettyConfig;
    let arg = args.iter().next().ok_or(
        Error::new(ErrorKind::UnexpectedArgs(0, 1))
    )?;
    if let Value::String(s) = arg {
        let tokens = TokenStream::new(&s);
        let stmt = Stmt::from_tokens(&tokens)?;
        Ok(Value::String(ron::ser::to_string_pretty(&stmt, PrettyConfig::new()).unwrap_or(
            String::from("invalid statement"))))
    } else {
        Err(Error::new(ErrorKind::InvalidType))
    }
}
