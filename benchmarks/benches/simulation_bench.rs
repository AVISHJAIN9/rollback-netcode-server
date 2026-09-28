use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rollback_netcode_server::simulation::state::WorldState;
use rollback_netcode_server::desync::detector::DesyncDetector;

fn bench_state_hash(c: &mut Criterion) {
    let state = WorldState::default();
    c.bench_function("blake3_state_checksum", |b| {
        b.iter(|| {
            DesyncDetector::compute_checksum(black_box(&state))
        })
    });
}

criterion_group!(benches, bench_state_hash);
criterion_main!(benches);
