use challenger_rs::position::Position;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn play_move_pawn_single_forward(c: &mut Criterion) {
    let start: Position = Position::new();
    c.bench_function("play_move_pawn_single_forward", |b| {
        b.iter(|| {
            let mut pos = start;
            pos.play_move("a2a3")
        })
    });
}

pub fn play_move_pawn_double_forward(c: &mut Criterion) {
    let start: Position = Position::new();
    c.bench_function("play_move_pawn_double_forward", |b| {
        b.iter(|| {
            let mut pos = start;
            pos.play_move("a2a4")
        })
    });
}

pub fn play_move_w_pawn_promotion(c: &mut Criterion) {
    let start: Position = Position::from("rnbqkbnr/pPpppppp/8/8/8/8/P1PPPPPP/RNBQKBNR w - - 0 1");
    c.bench_function("play_move_w_pawn_promotion", |b| {
        b.iter(|| {
            let mut pos = start;
            pos.play_move("b7c8N")
        })
    });
}

pub fn play_move_w_pawn_passant(c: &mut Criterion) {
    let start: Position =
        Position::from("rnbqkbnr/pppppp1p/8/5Pp1/8/8/PPPPP1PP/RNBQKBNR w - g6 0 1");
    c.bench_function("play_move_w_pawn_passant", |b| {
        b.iter(|| {
            let mut pos = start;
            pos.play_move("f5g6")
        })
    });
}

pub fn play_move_initial_knight(c: &mut Criterion) {
    let start: Position = Position::new();
    c.bench_function("play_move_initial_knight", |b| {
        b.iter(|| {
            let mut pos = start;
            pos.play_move("b1c3")
        })
    });
}

pub fn play_move_w_kingside_castle(c: &mut Criterion) {
    let start: Position = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w - - 0 1");
    c.bench_function("play_move_w_kingside_castle", |b| {
        b.iter(|| {
            let mut pos = start;
            pos.play_move("e1g1")
        })
    });
}

pub fn build_position(c: &mut Criterion) {
    let start: Position = Position::new();
    c.bench_function("construct_position", |b| {
        b.iter(|| {
            let pos: Position = Position::new();
            assert_eq!(pos, start);
        })
    });
}

criterion_group! {
    name = play_move;
    config = Criterion::default().sample_size(300);
    targets = play_move_pawn_single_forward, play_move_pawn_double_forward, play_move_w_pawn_promotion, play_move_w_pawn_passant, play_move_initial_knight, play_move_w_kingside_castle, build_position
}
criterion_main!(play_move);
