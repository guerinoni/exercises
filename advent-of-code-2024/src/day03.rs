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

#[must_use]
pub fn solve(input: &str) -> (u32, u32) {
    let mut left = 0;
    let mut right = 3;

    let mut p1 = 0;

    loop {
        // println!("left: {}, right: {}", left, right);
        if right >= input.len() {
            break;
        }

        let mul = &input[left..right];
        // println!("mul: {}", mul);
        if !mul.eq("mul") {
            // shift right by 1 and restart looking for `mul`
            left += 1;
            right += 1;
            continue;
        }

        // println!("found mul");

        //  found `mul`, now look for open bracket
        left += 3;
        right = left + 1;

        if !input[left..right].eq("(") {
            // don't match to open bracket, so restart looking for `mul`
            right = left + 3;
            continue;
        }

        left += 1;
        right += 1;

        let mut counter = 0;
        // parse the numbers until we find `,`
        loop {
            if input[right..=right].eq(",") {
                break;
            }

            counter += 1;
            if counter > 2 {
                break;
            }

            right += 1;
        }

        if counter > 2 {
            // number can be at most 3 digits, so restart looking for `mul`
            right = left + 3;
            continue;
        }

        // found `,`, parse number
        let Ok(x) = input[left..right].parse::<u32>() else {
            // not all characters are numbers, so restart looking for `mul`
            right = left + 3;
            continue;
        };

        left = right + 1;
        right = left + 1;

        counter = 0;

        // parse the numbers until we find `)`
        loop {
            if input[right..=right].eq(")") {
                break;
            }

            counter += 1;
            if counter > 2 {
                break;
            }

            right += 1;
        }

        if counter > 2 {
            // number can be at most 3 digits, so restart looking for `mul`
            right = left + 3;
            continue;
        }

        // found `)`, parse number
        let Ok(y) = input[left..right].parse::<u32>() else {
            // not all characters are numbers, so restart looking for `mul`
            right = left + 3;
            continue;
        };

        left = right + 1;
        right = left + 3;

        // println!("doing -> x: {}, y: {}", x, y);
        p1 += x * y;
    }

    (p1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "mul(2,4)";
        assert_eq!(solve(input), (8, 0));

        let input = "mul(10,40)";
        assert_eq!(solve(input), (400, 0));

        let input = "mul(101,401)";
        assert_eq!(solve(input), (40501, 0));

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
        assert_eq!(solve(input), (161, 0));
    }

    #[test]
    fn real() {
        let (p1, p2) = solve(include_str!("./testdata/day03"));
        assert_eq!(p1, 190604937);
    }
}
