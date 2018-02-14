#[macro_use]
extern crate criterion;
extern crate kilac;

use criterion::Criterion;
use kilac::calc::lexer::lex;
use kilac::calc::parser::parse;
use kilac::calc::calculate;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lexer interpoloi", |b| b.iter(|| lex("max(interpoloi(max([(a-0),0.5*med((.a*..mukana-0))]),max((.a*..mukana-0)),5,0.5*med((.a*..mukana-0))))")));
    c.bench_function("ast min", |b| b.iter(|| calculate("min(5, 2)".into())));
    c.bench_function("ast minmax", |b| b.iter(|| calculate("min(5, max(2, -2))".into())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
