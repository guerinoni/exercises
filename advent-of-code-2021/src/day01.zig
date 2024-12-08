// --- Day 1: Sonar Sweep ---

// You're minding your own business on a ship at sea when the overboard alarm goes off! You rush to see if you can help. Apparently, one of the Elves tripped and accidentally sent the sleigh keys flying into the ocean!

// Before you know it, you're inside a submarine the Elves keep ready for situations like this. It's covered in Christmas lights (because of course it is), and it even has an experimental antenna that should be able to track the keys if you can boost its signal strength high enough; there's a little meter that indicates the antenna's signal strength by displaying 0-50 stars.

// Your instincts tell you that in order to save Christmas, you'll need to get all fifty stars by December 25th.

// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

// As the submarine drops below the surface of the ocean, it automatically performs a sonar sweep of the nearby sea floor. On a small screen, the sonar sweep report (your puzzle input) appears: each line is a measurement of the sea floor depth as the sweep looks further and further away from the submarine.

// For example, suppose you had the following report:

// 199
// 200
// 208
// 210
// 200
// 207
// 240
// 269
// 260
// 263

// This report indicates that, scanning outward from the submarine, the sonar sweep found depths of 199, 200, 208, 210, and so on.

// The first order of business is to figure out how quickly the depth increases, just so you know what you're dealing with - you never know if the keys will get carried into deeper water by an ocean current or a fish or something.

// To do this, count the number of times a depth measurement increases from the previous measurement. (There is no measurement before the first measurement.) In the example above, the changes are as follows:

// 199 (N/A - no previous measurement)
// 200 (increased)
// 208 (increased)
// 210 (increased)
// 200 (decreased)
// 207 (increased)
// 240 (increased)
// 269 (increased)
// 260 (decreased)
// 263 (increased)

// In this example, there are 7 measurements that are larger than the previous measurement.

// How many measurements are larger than the previous measurement?

const std = @import("std");

const Result = struct {
    part1: u32,
    part2: u32,
};

pub fn solve(input: []const u8) !Result {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var lines = std.mem.tokenize(u8, input, "\n");
    var nums = std.ArrayList(u32).init(allocator);
    defer nums.deinit();

    var part1: u32 = 0;
    var part2: u32 = 0;

    var prev: u32 = (1 << 32) - 1; // put ths at max value to make sure the first number is always larger

    while (lines.next()) |line| {
        part2 += 2;
        const num = try std.fmt.parseInt(u32, line, 10);
        if (num > prev) {
            part1 += 1;
        }

        prev = num;
        try nums.append(num);
    }

    return Result{ .part1 = part1, .part2 = part2 };
}

const expect = @import("std").testing.expect;

test "part 1 sample" {
    const input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    const expected = 7;

    const result = try solve(input);
    try expect(result.part1 == expected);
}
