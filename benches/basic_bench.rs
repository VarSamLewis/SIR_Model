// benches/basic_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn basic_benchmark(c: &mut Criterion) {
    c.bench_function("basic", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, basic_benchmark);
criterion_main!(benches);