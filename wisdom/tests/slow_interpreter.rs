use interpreter::error::Error;
use wisdom::ast::Value;
use wisdom::interpreter::*;
use wisdom::interpreter::error::ErrorKind::UndefinedVar;

fn run_script(script: &str, expect: Result<Value, Error>) {
    let mut itp = SlowInterpreter::new();
    let result = itp.eval_script(script);
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
while a < 10 {
    a = a + 1;
}
a
"#;
    run_script(script, Ok(Value::Int(10)));
}

#[test]
fn test_scope() {
    let script = r#"
a = 10;
while a > 0 {
    b = 1;
    a = a - b;
}
b
"#;
    run_script(script, Err(Error::new(UndefinedVar("b".to_string()))));
}

#[test]
fn test_multi_binop() {
    let script = r#"
a = 10;
b = a < 10 && a > 5;
b
"#;
    run_script(script, Ok(Value::Bool(false)));
}

#[test]
fn test_multi_in_if() {
    let script = r#"
a = 0;
if a < 10 && a > 5 {
    a = 10;
}
a
"#;
    run_script(script, Ok(Value::Int(0)));
}