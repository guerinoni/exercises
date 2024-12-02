use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_of_code_2024::day01;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = r"3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
    c.bench_function("day01", |b| b.iter(|| day01::solve(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);