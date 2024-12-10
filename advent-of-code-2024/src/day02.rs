// https://adventofcode.com/2024/day/2

fn is_safe(v: impl Iterator<Item = u32> + Clone) -> (bool, usize) {
    // take first 5 elements to decide if the list is increasing or decreasing
    let iter = v.clone().take(5);
    let count_inc = iter
        .clone()
        .zip(iter.skip(1))
        .filter(|(a, b)| a < b)
        .count();
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
        assert_eq!(
            is_safe([27, 24, 26, 29, 30, 33, 36].iter().copied()),
            (false, 0)
        );
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
}
