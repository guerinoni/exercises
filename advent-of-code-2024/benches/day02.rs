use advent_of_code_2024::day02;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = r"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";
    c.bench_function("day02", |b| b.iter(|| day02::solve(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
