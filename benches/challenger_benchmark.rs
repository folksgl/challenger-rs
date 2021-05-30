use challenger_rs::position;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sq_to_bitboard", |b| {
        b.iter(|| position::sq_to_bitboard(black_box('a'), black_box('2')))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
