use advent_of_code_2024::day03;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    c.bench_function("day03", |b| b.iter(|| day03::solve(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
