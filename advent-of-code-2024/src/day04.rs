// --- Day 4: Ceres Search ---

// "Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

// As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

// This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:

// ..X...
// .SAMX.
// .A..A.
// XMAS.S
// .X....

// The actual word search will be full of letters instead. For example:

// MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX

// In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

// ....XXMAS.
// .SAMXMS...
// ...S..A...
// ..A.A.MS.X
// XMASAMX.MM
// X.....XA.A
// S.S.S.S.SS
// .A.A.A.A.A
// ..M.M.M.MM
// .X.X.XMASX

// Take a look at the little Elf's word search. How many times does XMAS appear?

// --- Part Two ---

// The Elf looks quizzically at you. Did you misunderstand the assignment?

// Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:

// M.S
// .A.
// M.S

// Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.

// Here's the same example from before, but this time all of the X-MASes have been kept instead:

// .M.S......
// ..A..MSMS.
// .M.S.MAA..
// ..A.ASMSM.
// .M.S.M....
// ..........
// S.S.S.S.S.
// .A.A.A.A..
// M.M.M.M.M.
// ..........

// In this example, an X-MAS appears 9 times.

// Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear?

#[allow(clippy::cast_possible_wrap)]
fn try_right(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 3 >= matrix[y].len() {
        return false;
    }

    matrix[y][x + 1] == 'M' && matrix[y][x + 2] == 'A' && matrix[y][x + 3] == 'S'
}

#[allow(clippy::cast_possible_wrap)]
fn try_left(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if (x as isize) - 3 < 0 {
        return false;
    }

    matrix[y][x - 1] == 'M' && matrix[y][x - 2] == 'A' && matrix[y][x - 3] == 'S'
}

#[allow(clippy::cast_possible_wrap)]
fn try_down(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if y + 3 >= matrix.len() {
        return false;
    }

    matrix[y + 1][x] == 'M' && matrix[y + 2][x] == 'A' && matrix[y + 3][x] == 'S'
}

#[allow(clippy::cast_possible_wrap)]
fn try_up(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if (y as isize) - 3 < 0 {
        return false;
    }

    matrix[y - 1][x] == 'M' && matrix[y - 2][x] == 'A' && matrix[y - 3][x] == 'S'
}
#[allow(clippy::cast_possible_wrap)]
fn try_right_down(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 3 >= matrix[y].len() || y + 3 >= matrix.len() {
        return false;
    }

    matrix[y + 1][x + 1] == 'M' && matrix[y + 2][x + 2] == 'A' && matrix[y + 3][x + 3] == 'S'
}

#[allow(clippy::cast_possible_wrap)]
fn try_left_down(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if (x as isize) - 3 < 0 || y + 3 >= matrix.len() {
        return false;
    }

    matrix[y + 1][x - 1] == 'M' && matrix[y + 2][x - 2] == 'A' && matrix[y + 3][x - 3] == 'S'
}
#[allow(clippy::cast_possible_wrap)]
fn try_right_up(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 3 >= matrix[y].len() || (y as isize) - 3 < 0 {
        return false;
    }

    matrix[y - 1][x + 1] == 'M' && matrix[y - 2][x + 2] == 'A' && matrix[y - 3][x + 3] == 'S'
}
#[allow(clippy::cast_possible_wrap)]
fn try_left_up(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if (x as isize) - 3 < 0 || (y as isize) - 3 < 0 {
        return false;
    }

    matrix[y - 1][x - 1] == 'M' && matrix[y - 2][x - 2] == 'A' && matrix[y - 3][x - 3] == 'S'
}

// It returs the number of XMAS found
fn find_xmas(matrix: &[Vec<char>], x: usize, y: usize) -> u32 {
    let right: u32 = try_right(matrix, x, y).into();
    let left: u32 = try_left(matrix, x, y).into();
    let down: u32 = try_down(matrix, x, y).into();
    let up: u32 = try_up(matrix, x, y).into();
    let rd: u32 = try_right_down(matrix, x, y).into();
    let ld: u32 = try_left_down(matrix, x, y).into();
    let ru: u32 = try_right_up(matrix, x, y).into();
    let lu: u32 = try_left_up(matrix, x, y).into();

    right + left + down + up + rd + ld + ru + lu
}

#[must_use]
pub fn solve(input: &str) -> (u32, u32) {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut p1 = 0;
    let (mut x, mut y) = (0, 0);

    let horizontal_len = matrix[0].len();
    let vertical_len = matrix.len();

    loop {
        if y >= vertical_len {
            // finished
            break;
        }

        if x >= horizontal_len {
            // next line
            x = 0;
            y += 1;
            continue;
        }

        let ch = matrix[y][x];
        if ch == 'X' {
            p1 += find_xmas(&matrix, x, y);
        }

        x += 1;
    }

    (p1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        assert_eq!(solve(input), (18, 0));
    }
}
