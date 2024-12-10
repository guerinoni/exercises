// https://adventofcode.com/2021/day/1

const std = @import("std");

const Result = struct {
    part1: u32,
    part2: u32,
};

pub fn solve(input: []const u8) !Result {
    if (input.len == 0) {
        return Result{ .part1 = 0, .part2 = 0 };
    }

    var lines = std.mem.tokenize(u8, input, "\n");

    var part1: u32 = 0;
    var part2: u32 = 0;

    var prev: u32 = (1 << 32) - 1;
    var prev1: u32 = (1 << 32) - 1;
    var prev2: u32 = (1 << 32) - 1;
    var prev3: u32 = (1 << 32) - 1;

    var last_sum: u32 = (1 << 32) - 1;

    while (lines.next()) |line| {
        const num = try std.fmt.parseInt(u32, line, 10);
        if (num > prev) {
            part1 += 1;
        }

        prev = num;

        if ((prev1 != (1 << 32) - 1) and (prev2 != (1 << 32) - 1) and (prev3 != (1 << 32) - 1)) {
            const sum = prev1 + prev2 + prev3;
            if (sum > last_sum) {
                part2 += 1;
            }

            last_sum = sum;
        }

        prev3 = prev2;
        prev2 = prev1;
        prev1 = num;
    }

    // we need to do the last calculation for part2 because the last iteration of the loop
    // we do the previous check and update the triplets with latest value read.
    const sum = prev1 + prev2 + prev3;
    if (sum > last_sum) {
        part2 += 1;
    }

    return Result{ .part1 = part1, .part2 = part2 };
}

const expect = @import("std").testing.expect;

test "sample" {
    const input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    const result = try solve(input);
    try expect(result.part1 == 7);
    try expect(result.part2 == 5);
}
