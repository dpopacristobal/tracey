use criterion::{criterion_group, criterion_main, Criterion};

extern crate rey_skytracer;

use rey_skytracer::render::render;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render random 1", |b| b.iter(|| render()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
