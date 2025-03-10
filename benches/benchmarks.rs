use criterion::{black_box, criterion_group, criterion_main, Criterion};
use libchessticot::{Planner, Player, Position};
fn planner_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("planner move search");
    group.sample_size(10);
    let position =
        Position::from_fen("r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 1");
    group.bench_function("offer planner move", |b| {
        b.iter(|| Planner {}.offer_move(black_box(&position)))
    });
}

fn perft_3_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("perft 3");
    let position = Position::initial();
    group.bench_function("perft 3", |b| b.iter(|| position.perft(3)));
}

criterion_group!(benches, planner_benchmark, perft_3_benchmark);
criterion_main!(benches);
