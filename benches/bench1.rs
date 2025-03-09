use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn loop_add(n: u64) {
    let mut count: u64 = 1u64;
    while count < n {
        count += 1;
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("loop_add", |b| b.iter(|| loop_add(black_box(2000000u64))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
