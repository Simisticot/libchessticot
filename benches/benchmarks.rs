use criterion::{black_box, criterion_group, criterion_main, Criterion};
use libchessticot::{Planner, Player, Position};
fn minimax_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("minimax");
    group.sample_size(10);
    let position =
        Position::from_fen("r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 1");
    group.bench_function("minimax 3", |b| {
        b.iter(|| Planner {}.offer_move(black_box(&position)))
    });
}

criterion_group!(benches, minimax_benchmark);
criterion_main!(benches);
