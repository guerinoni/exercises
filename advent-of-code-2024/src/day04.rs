// https://adventofcode.com/2024/day/4

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

#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub fn solve(input: &str) -> (u32, u32) {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    if matrix.is_empty() {
        return (0, 0);
    }

    let (mut p1, mut p2) = (0, 0);
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

        // take the center of X-MAS
        // A cannot be on the edges
        if ch == 'A' && x > 0 && y > 0 && x < horizontal_len - 1 && y < vertical_len - 1 {
            let top_left = matrix[y - 1][x - 1];
            let top_right = matrix[y - 1][x + 1];
            let bottom_left = matrix[y + 1][x - 1];
            let bottom_right = matrix[y + 1][x + 1];

            if (top_left == 'M' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'S')
                || (top_left == 'S'
                    && top_right == 'S'
                    && bottom_left == 'M'
                    && bottom_right == 'M')
                || (top_left == 'M'
                    && top_right == 'S'
                    && bottom_left == 'M'
                    && bottom_right == 'S')
                || (top_left == 'S'
                    && top_right == 'M'
                    && bottom_left == 'S'
                    && bottom_right == 'M')
            {
                p2 += 1;
            }
        }

        x += 1;
    }

    (p1, p2)
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

        assert_eq!(solve(input), (18, 9));
    }
}
