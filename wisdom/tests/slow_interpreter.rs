use interpreter::error::Error;
use wisdom::ast::Value;
use wisdom::interpreter::*;
use wisdom::interpreter::error::ErrorKind::UndefinedVar;

fn run_script(script: &str, expect: std::result::Result<Value, Error>) {
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

#[test]
fn test_return_stmt() {
    let script = r#"
fn max(a, b) {
    if a > b {
        return a;
    }
    return b;
}
"#.to_owned();
    let mut max_a = script.clone();
    max_a.push_str("max(20, 10)");
    let mut max_b = script.clone();
    max_b.push_str("max(10, 20);");
    run_script(&max_a, Ok(Value::Int(20)));
    run_script(&max_b, Ok(Value::Int(20)));
}