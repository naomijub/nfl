use criterion::{criterion_group, criterion_main, Criterion};
use nfl_rushing::reader::read_json;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rushing.json", |b| b.iter(|| read_json("rushing.json")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
