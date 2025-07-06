use std::env;

use compute_rcv::count_rcv;

use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rcv", |b| {
        b.iter(|| {
            count_rcv(
                &std::fs::canonicalize(format!(
                    "{}/../better_vote_data.mpack",
                    env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set")
                ))
                .expect("Failed to canonicalize vote data path"),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
