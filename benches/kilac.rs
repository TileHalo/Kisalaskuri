#[macro_use]
extern crate criterion;
extern crate kilac;

use criterion::Criterion;
use kilac::calc::lexer::lex;
use kilac::calc::parser::{parse, parse_fn};
use kilac::calc::parser::applicators::empty;
use kilac::calc::ctx::EmptyCtx;
use kilac::calc::{calculate, eval};

fn lexer(c: &mut Criterion) {
    c.bench_function("lexer interpoloi", |b| b.iter(|| lex("max(interpoloi(max([(a-0),0.5*med((.a*..mukana-0))]),max((.a*..mukana-0)),5,0.5*med((.a*..mukana-0))))")));
}

fn parser(c: &mut Criterion) {
    c.bench_function("ast min", |b| b.iter(|| parse(lex("min(5, 2)".into()))));
    c.bench_function("ast minmax", |b| b.iter(|| parse(lex("min(5, max(2, -2))".into()))));
    c.bench_function("ast empty min", |b| b.iter(|| parse_fn(lex("min(5, 2)".into()), empty, EmptyCtx)));
    c.bench_function("ast empty minmax", |b| b.iter(|| parse_fn(lex("min(5, max(2, -2))".into()), empty, EmptyCtx)));
}

fn evalb(c: &mut Criterion) {
    c.bench_function("calc min", |b| b.iter(|| calculate("min(5, 2)".into())));
    c.bench_function("calc minmax", |b| b.iter(|| calculate("min(5, max(2, -2))".into())));
    c.bench_function("calc empty min", |b| b.iter(|| eval(parse_fn(lex("min(5, 2)".into()), empty, EmptyCtx).ok().unwrap())));
    c.bench_function("calc empty minmax", |b| b.iter(|| eval(parse_fn(lex("min(5, max(2, -2))".into()), empty, EmptyCtx).ok().unwrap())));
}

criterion_group!(benches, lexer, parser, evalb);
criterion_main!(benches);
