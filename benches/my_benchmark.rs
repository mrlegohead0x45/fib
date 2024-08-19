use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fib::fib;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fib(black_box(20))));
    c.bench_function("fib 100,000", |b| b.iter(|| fib(black_box(100_000))));
    c.bench_function("fib 1,000,000", |b| b.iter(|| fib(black_box(1_000_000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
