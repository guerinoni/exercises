mod day01;
mod day02;
mod memory;

fn main() {
    tracing_subscriber::fmt().pretty().init();

    memory::print("allocated bytes before main");

    println!("Hello AoC 2024!");

    let (p1, p2) = day01::solve(include_str!("./testdata/day01"));
    println!("Day 01");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");

    memory::print("after day01");

    let (p1, p2) = day02::solve(include_str!("./testdata/day02"));
    println!("Day 02");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");

    memory::print("after day02");
}
