use challenger_rs::position::Position;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn play_move_pawn_single_forward(c: &mut Criterion) {
    c.bench_function("play_move_pawn_single_forward", |b| {
        b.iter(|| Position::new().play_move("a2a3"))
    });
}

criterion_group!(play_move, play_move_pawn_single_forward);
criterion_main!(play_move);
