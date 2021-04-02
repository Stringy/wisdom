use interpreter::error::Error;
use wisdom::ast::Value;
use wisdom::interpreter::*;
use wisdom::interpreter::error::ErrorKind::UndefinedVar;
use std::io;
use std::io::BufRead;

fn run_script(script: &str, expect: Result<Value, Error>) {
    let mut itp = SlowInterpreter::new();
    let cursor = io::Cursor::new(script);
    let mut result = Ok(Value::None);
    for line in cursor.lines() {
        let line = line.unwrap();
        result = itp.eval_line(&line);
    }
    assert_eq!(result, expect);
}

fn run_lines(itp: &mut SlowInterpreter, lines: &[&str], expect: Result<Value, Error>) {
    let mut result = Ok(Value::None);
    for line in lines {
        result = itp.eval_line(line);
    }
    assert_eq!(result, expect);
}

#[test]
fn test_simple_expression() {
    run_script("1 + 1;", Ok(Value::Int(2)));
}

#[test]
fn test_assignment() {
    let script = r#"
a = 123;
a
"#;
    run_script(script, Ok(Value::Int(123)));
}

#[test]
fn test_loop() {
    let script = r#"
a = 1;
while a < 10 { a = a + 1; }
a
"#;
    run_script(script, Ok(Value::Int(10)));
}

#[test]
fn test_scope() {
    let lines = &[
        "a = 10;",
        "while a > 0 { b = 1; a = a - b; }",
        "b"
    ];
    let mut itp = SlowInterpreter::new();
    run_lines(&mut itp, lines, Err(Error::new(UndefinedVar("b".to_string()))));
}

#[test]
fn test_multi_binop() {
    let lines = &[
        "a = 10;",
        "b = a < 10 && a > 5;",
        "b"
    ];
    let mut itp = SlowInterpreter::new();
    run_lines(&mut itp, lines, Ok(Value::Bool(false)));
}

#[test]
fn test_multi_in_if() {
    let lines = &[
        "a = 0;",
        "if a < 10 && a > 5 { a = 10; }",
        "a"
    ];
    let mut itp = SlowInterpreter::new();
    run_lines(&mut itp, lines, Ok(Value::Int(0)));
}