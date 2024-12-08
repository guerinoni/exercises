const std = @import("std");

const day01 = @import("day01.zig");

const input01 = @embedFile("./testdata/day01");

pub fn main() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Hello AoC 2024!\n", .{});

    const result = try day01.solve(input01);
    try stdout.print("Day 01\n", .{});
    try stdout.print("  Part 1: {}\n", .{result.part1});
    try stdout.print("  Part 2: {}\n", .{result.part2});

    try bw.flush(); // don't forget to flush!
}
