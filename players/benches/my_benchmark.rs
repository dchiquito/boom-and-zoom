use baz_core::{Board, Color};
use baz_players::{GeniusHeuristic, GoFastHeuristic, GoFasterHeuristic, Heuristic, NaiveHeuristic};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::default();
    // TODO shuffle the board
    let mut go_fast = GoFastHeuristic();
    c.bench_function("go fast heuristic", |b| {
        b.iter(|| {
            go_fast.evaluate(&board, &Color::White);
        })
    });
    let mut go_faster = GoFasterHeuristic();
    c.bench_function("go faster heuristic", |b| {
        b.iter(|| {
            go_faster.evaluate(&board, &Color::White);
        })
    });
    let mut genius = GeniusHeuristic();
    c.bench_function("genius heuristic", |b| {
        b.iter(|| {
            genius.evaluate(&board, &Color::White);
        })
    });
    let mut naive = NaiveHeuristic();
    c.bench_function("naive heuristic", |b| {
        b.iter(|| {
            naive.evaluate(&board, &Color::White);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
