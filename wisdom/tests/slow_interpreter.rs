use wisdom::ast::value::Value;
use wisdom::interpreter::*;

fn run_lines(itp: &mut SlowInterpreter, lines: &[&str], expect: Value) {
    let mut result= Value::None;
    for line in lines {
        result = itp.eval_line(line).unwrap();
    }
    assert_eq!(result, expect);
}

#[test]
fn test_simple_expression() {
    let mut itp = SlowInterpreter::new();
    run_lines(&mut itp, &["1 + 1;"], Value::Int(2));
}

#[test]
fn test_assignment() {
    let mut itp = SlowInterpreter::new();
    run_lines(&mut itp, &["a = 123;", "a"], Value::Int(123));
}

#[test]
fn test_loop() {
    let mut itp = SlowInterpreter::new();
    let lines = &[
        "a = 1;",
        "while a < 10 { a = a + 1; }",
        "a"
    ];
    run_lines(&mut itp, lines, Value::Int(10));
}