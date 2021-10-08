use criterion::{black_box, criterion_group, criterion_main, Criterion};
use subprocess::*;
use std::error::Error;

fn mytest() {
    Exec::shell("find ").capture().unwrap();
}
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| mytest()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
