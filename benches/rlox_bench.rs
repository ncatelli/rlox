use criterion::{criterion_group, criterion_main, Criterion};
extern crate librlox;
extern crate parcel;
use librlox::ast::token::Token;
use librlox::parser::expression_parser::expression;
use librlox::scanner::Scanner;

use parcel::prelude::v1::*;

fn scan_tokens_benchmark(c: &mut Criterion) {
    let s = Scanner::new(";*.\"hello world\"123.4".to_string());
    c.bench_function("scan tokens", |b| {
        b.iter(|| {
            let _tokens = s.scan_tokens();
        })
    });
}

fn parse_expr_benchmark(c: &mut Criterion) {
    let s = Scanner::new("1 * ( 2 + 3 ) - 4".to_string());
    let token_iter = s.into_iter();
    let tokens: Vec<Token> = token_iter
        .map(|tok| match tok {
            Ok(tok) => tok,
            Err(e) => panic!("{}", e),
        })
        .collect();

    c.bench_function("parse expressions", |b| {
        b.iter(|| {
            let _expr = expression().parse(&tokens);
        })
    });
}

criterion_group!(benches, scan_tokens_benchmark, parse_expr_benchmark);
criterion_main!(benches);
