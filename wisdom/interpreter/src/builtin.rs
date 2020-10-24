use ast::value::Value;

pub fn exists(name: &String) -> bool {
    match name.as_str() {
        "print" => true,
        _ => false
    }
}

pub fn run(name: &String, args: Vec<Value>) -> Result<Value, ()> {
    match name.as_str() {
        "print" => print(args),
        _ => panic!("no such builtin function: {}", name)
    }
}

pub fn print(args: Vec<Value>) -> Result<Value, ()> {
    for arg in args {
        print!("{}", arg);
    }
    print!("\n");
    Ok(Value::None)
}
