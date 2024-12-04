// --- Day 2: Red-Nosed Reports ---

// Fortunately, the first location The Historians want to search isn't a long walk from the Chief Historian's office.

// While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they still talk about the time Rudolph was saved through molecular synthesis from a single electron.

// They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.

// The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9

// This example data contains six reports each containing five levels.

// The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

//     The levels are either all increasing or all decreasing.
//     Any two adjacent levels differ by at least one and at most three.

// In the example above, the reports can be found safe or unsafe by checking those rules:

//     7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
//     1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
//     9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
//     1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
//     8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
//     1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

// So, in this example, 2 reports are safe.

// Analyze the unusual data from the engineers. How many reports are safe?

// --- Part Two ---

// The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.

// The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!

// Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

// More of the above example's reports are now safe:

//     7 6 4 2 1: Safe without removing any level.
//     1 2 7 8 9: Unsafe regardless of which level is removed.
//     9 7 6 2 1: Unsafe regardless of which level is removed.
//     1 3 2 4 5: Safe by removing the second level, 3.
//     8 6 4 4 1: Safe by removing the third level, 4.
//     1 3 6 7 9: Safe without removing any level.

// Thanks to the Problem Dampener, 4 reports are actually safe!

// Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe?

fn is_safe<'a>(v: impl Iterator<Item = u32> + Clone) -> (bool, usize) {
    // take first 5 elements to decide if the list is increasing or decreasing
    let iter = v.clone().take(5);
    let count_inc = iter.clone().zip(iter.clone().skip(1)).filter(|(a, b)| a < b).count();
    let is_increasing = count_inc > 2;

    let mut idx = 0;
    (
        v.clone().enumerate().zip(v.skip(1)).all(|((index, a), b)| {
            let diff = b.abs_diff(a);
            idx = index;
            ((is_increasing && a <= b) || (!is_increasing && a >= b)) && diff > 0 && diff <= 3
        }),
        idx, // return index of the error
    )
}

#[must_use]
pub fn solve(input: &str) -> (i32, i32) {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .map(str::parse)
            .filter_map(Result::ok)
            .collect::<Vec<u32>>();

        let (safe, idx_error) = is_safe(nums.iter().copied()); // zero cost copy those types

        if safe {
            p1 += 1;

            // skip the check of dampener, is already safe
            continue;
        }

        for i in idx_error..nums.len() {
            let part1 = &nums[..i];
            let part2 = &nums[i + 1..];

            let (safe, _) = is_safe(part1.iter().chain(part2.iter()).copied());
            if safe {
                p2 += 1;

                // if we find a safe report, we can break the loop
                break;
            }
        }
    }

    (p1, p2 + p1) // concatenate the safe reports from part 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safety() {
        assert_eq!(is_safe([7, 6, 4, 2, 1].iter().copied()), (true, 3));
        assert_eq!(is_safe([1, 2, 7, 8, 9].iter().copied()), (false, 1));
        assert_eq!(is_safe([9, 7, 6, 2, 1].iter().copied()), (false, 2));
        assert_eq!(is_safe([1, 3, 2, 4, 5].iter().copied()), (false, 1));
        assert_eq!(is_safe([8, 6, 4, 4, 1].iter().copied()), (false, 2));
        assert_eq!(is_safe([27, 24, 26, 29, 30, 33, 36].iter().copied()), (false, 0));
    }

    #[test]
    fn sample() {
        let input = r"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";
        assert_eq!(solve(input), (2, 4));
    }

    #[test]
    fn real() {
        let input = include_str!("./testdata/day02");
        assert_eq!(solve(input), (287, 354));
    }
}
