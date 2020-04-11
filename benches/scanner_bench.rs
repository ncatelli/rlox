use criterion::{criterion_group, criterion_main, Criterion};
extern crate librlox;
use librlox::scanner::Scanner;

fn scan_tokens_benchmark(c: &mut Criterion) {
    let s = Scanner::new(";*.\"hello world\"123.4".to_string());
    c.bench_function("scan tokens", |b| {
        b.iter(|| {
            let _tokens = s.scan_tokens();
        })
    });
}

criterion_group!(benches, scan_tokens_benchmark);
criterion_main!(benches);
