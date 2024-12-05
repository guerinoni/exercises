// --- Day 3: Mull It Over ---

// "Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.

// The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"

// The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!

// It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.

// However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.

// For example, consider the following section of corrupted memory:

// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

// Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).

// Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications?

// --- Part Two ---

// As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.

// There are two new instructions you'll need to handle:

//     The do() instruction enables future mul instructions.
//     The don't() instruction disables future mul instructions.

// Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.

// For example:

// xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))

// This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.

// This time, the sum of the results is 48 (2*4 + 8*5).

// Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications?

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

    #[test]
    fn real() {
        let (p1, p2) = solve(include_str!("./testdata/day03"));
        assert_eq!(p1, 190604937);
        assert_eq!(p2, 82857512);
    }
}
