use compute_rcv::count_rcv;

use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rcv", |b| b.iter(|| count_rcv()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
