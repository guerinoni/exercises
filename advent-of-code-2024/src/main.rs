mod day01;

fn main() {
    println!("Hello AoC 2024!");

    let (p1,p2) = day01::solve(include_str!("./testdata/day01"));
    println!("Day 01");
    println!("  Part 1: {}", p1);
    println!("  Part 2: {}", p2);
}
