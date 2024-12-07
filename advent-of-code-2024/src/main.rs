mod day01;
mod day02;
mod day03;
mod day04;
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

    let (p1, p2) = day03::solve(include_str!("./testdata/day03"));
    println!("Day 03");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");

    memory::print("after day03");

    let (p1, p2) = day04::solve(include_str!("./testdata/day04"));
    println!("Day 04");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");

    memory::print("after day04");
}
