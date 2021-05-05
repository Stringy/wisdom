use criterion::{black_box, criterion_group, criterion_main, Criterion};
use interpreter::{SlowInterpreter, Interpreter};

fn bench_simple_expression(c: &mut Criterion) {
    let mut intp = SlowInterpreter::new();
    c.bench_function("simple expr", |b| b.iter(|| intp.eval_script(black_box("1 + 5"))));
}

fn bench_complex_expression(c: &mut Criterion) {
    let mut intp = SlowInterpreter::new();
    let script = r#"
let a = 10;
let b = 20;
a * b - 200 + 1345
"#;
    c.bench_function("complex expr", |b| b.iter(|| intp.eval_script(black_box(script))));
}

fn bench_function_call(c: &mut Criterion) {
    let mut intp = SlowInterpreter::new();
    let func = "fn foo() {}";
    intp.eval_script(func).unwrap();
    c.bench_function("function call", |b| b.iter(|| intp.eval_script(black_box("foo()"))));
}

criterion_group!(
    benches,
    bench_simple_expression,
    bench_complex_expression,
    bench_function_call
);

criterion_main!(benches);