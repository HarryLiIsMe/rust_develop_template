use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn loop_sub() {
    let mut count: u64 = black_box(2000000u64);
    while count == 0u64 {
        count -= 1;
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("loop_sub", |b| b.iter(|| loop_sub()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
