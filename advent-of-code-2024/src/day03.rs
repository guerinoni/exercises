// https://adventofcode.com/2024/day/3

// It returns the multiplication result and the position of left index
// useful to continue parsing from the correct position without wasted cycles.
fn parse_mul(input: &str) -> (u32, usize) {
    let mut left = 0;
    let mut right = 3;

    let def = &input[left..right];
    if !def.eq("mul") {
        // not mul, so continue other check from here
        return (0, left);
    }

    // found `mul`, now look for open bracket
    left += 3;
    right = left + 1;

    let open = &input[left..right];
    if !open.eq("(") {
        // don't match to open bracket, so check other from here
        return (0, left);
    }

    // found `(`, so parse the numbers
    left += 1;
    right += 1;

    let mut counter = 0;
    loop {
        let comma = &input[right..=right];
        if comma.eq(",") {
            break;
        }

        counter += 1;
        if counter > 2 {
            break;
        }

        right += 1;
    }

    if counter > 2 {
        // number can be at most 3 digits, so check other from here
        return (0, left);
    }

    let num = &input[left..right];
    let Ok(x) = num.parse::<u32>() else {
        // not all characters are numbers, so check other from here
        return (0, left);
    };

    // look for other number
    left = right + 1;
    right = left + 1;

    // parse the number until we find close bracket
    counter = 0;
    loop {
        let close = &input[right..=right];
        if close.eq(")") {
            break;
        }

        counter += 1;
        if counter > 2 {
            break;
        }

        right += 1;
    }

    if counter > 2 {
        // number can be at most 3 digits, so check other from here
        return (0, left);
    }

    let num = &input[left..right];
    let Ok(y) = num.parse::<u32>() else {
        // not all characters are numbers, so check other from here
        return (0, left);
    };

    left = right;

    (x * y, left)
}

#[must_use]
pub fn solve(input: &str) -> (u32, u32) {
    let mut left = 0;
    let mut exec = true;
    let (mut total_sum, mut active_sum) = (0, 0);

    // left + 3 is minimum length of valid instruction, in our case "mul"
    while left + 3 < input.len() {
        let slice = &input[left..];

        let (res, consumed) = parse_mul(slice);

        if res > 0 {
            total_sum += res;
            if exec {
                active_sum += res;
            }
            left += consumed;
        } else if slice.starts_with("do()") {
            exec = true;
            left += 4; // "do()" length
        } else if slice.starts_with("don't()") {
            exec = false;
            left += 7; // "don't()" length
        } else {
            // If no patterns match, move forward by one character
            left += 1;
        }
    }

    (total_sum, active_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "mul(2,4)";
        assert_eq!(solve(input), (8, 8));

        let input = "mul(10,40)";
        assert_eq!(solve(input), (400, 400));

        let input = "mul(101,401)";
        assert_eq!(solve(input), (40501, 40501));

        let input = "mul(1011,1401)"; // invalid
        assert_eq!(solve(input), (0, 0));

        let input = "mul(2, 4)";
        assert_eq!(solve(input), (0, 0));

        let input = "mul(2 ,4)";
        assert_eq!(solve(input), (0, 0));

        let input = "xmul(2, 4)";
        assert_eq!(solve(input), (0, 0));

        let input = "mulo(2, 4)";
        assert_eq!(solve(input), (0, 0));

        let input = "mal(2, 4)";
        assert_eq!(solve(input), (0, 0));

        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve(input), (161, 161));

        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve(input), (161, 48));
    }
}
