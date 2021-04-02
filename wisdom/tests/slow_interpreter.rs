use interpreter::error::Error;
use wisdom::ast::Value;
use wisdom::interpreter::*;
use wisdom::interpreter::error::ErrorKind::UndefinedVar;

fn run_lines(itp: &mut SlowInterpreter, lines: &[&str], expect: Result<Value, Error>) {
    let mut result = Ok(Value::None);
    for line in lines {
        result = itp.eval_line(line);
    }
    assert_eq!(result, expect);
}

#[test]
fn test_simple_expression() {
    let mut itp = SlowInterpreter::new();
    run_lines(&mut itp, &["1 + 1;"], Ok(Value::Int(2)));
}

#[test]
fn test_assignment() {
    let mut itp = SlowInterpreter::new();
    run_lines(&mut itp, &["a = 123;", "a"], Ok(Value::Int(123)));
}

#[test]
fn test_loop() {
    let mut itp = SlowInterpreter::new();
    let lines = &[
        "a = 1;",
        "while a < 10 { a = a + 1; }",
        "a"
    ];
    run_lines(&mut itp, lines, Ok(Value::Int(10)));
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