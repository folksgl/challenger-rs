use challenger_rs::position;
use challenger_rs::position::Position;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// Cheat sheet
//
// Save a baseline:
// 'cargo bench --bench challenger_benchmark -- --save-baseline main'
//
// Compare baselines:
// 'cargo bench --bench challenger_benchmark -- --load-benchmark base_2 --baseline base_1
pub fn play_moves(c: &mut Criterion) {
    let start: Position =
        Position::from("1nb1k2r/ppppppp1/r4n2/P1bq4/2BQ3p/R4N2/1PPPPPPP/1NB1K2R w - - 0 1");
    let moves: Vec<u16> = vec![
        //"g2g4", "h4g3", "f3e5", "b7b5", "a5b6", "f6e4", "e1g1", "e8g8", "c4c6", "c5a3", "d4e4",
        //"d5e5", "f1d1", "f8d8", "g1g2", "g8f1", "b6c7", "g3f2", "c7b2N", "f2f1n",
        1934, 34207, 2325, 2161, 35424, 1837, 388, 4028, 2714, 1058, 1819, 2339, 197, 3837, 902,
        382, 3241, 854, 17010, 16717,
    ];
    let bit_moves: Vec<u16> = vec![];

    c.bench_function("play_moves", |b| {
        b.iter(|| {
            let mut pos = start;
            for mov in moves.iter() {
                pos.play_move(*mov)
            }
        })
    });
}

criterion_group! {
    name = play_move;
    config = Criterion::default().sample_size(300);
    targets = play_moves
}
criterion_main!(play_move);
