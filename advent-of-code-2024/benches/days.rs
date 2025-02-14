use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day01(c: &mut Criterion) {
    let input = r"3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
    c.bench_function("day01", |b| {
        b.iter(|| advent_of_code_2024::day01::solve(black_box(input)));
    });
}

pub fn day02(c: &mut Criterion) {
    let input = r"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";
    c.bench_function("day02", |b| {
        b.iter(|| advent_of_code_2024::day02::solve(black_box(input)));
    });
}

pub fn day03(c: &mut Criterion) {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    c.bench_function("day03", |b| {
        b.iter(|| advent_of_code_2024::day03::solve(black_box(input)));
    });
}

pub fn day04(c: &mut Criterion) {
    let input = include_str!("../src/testdata/day04");
    c.bench_function("day04", |b| {
        b.iter(|| advent_of_code_2024::day04::solve(black_box(input)));
    });
}

pub fn day05(c: &mut Criterion) {
    let input = include_str!("../src/testdata/day05");
    c.bench_function("day05", |b| {
        b.iter(|| advent_of_code_2024::day05::solve(black_box(input)));
    });
}

criterion_group!(benches, day01, day02, day03, day04, day05);
criterion_main!(benches);
