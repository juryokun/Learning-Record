use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sample02::hash;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hash 2", |b| {
        b.iter(|| {
            let n = black_box(2);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
