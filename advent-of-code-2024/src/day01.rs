// https://adventofcode.com/2024/day/1
#[must_use]
pub fn solve(input: &str) -> (u32, u32) {
    let length = input.lines().count();

    let mut list_1 = Vec::with_capacity(length);
    let mut list_2 = Vec::with_capacity(length);

    let mut counters = std::collections::HashMap::with_capacity(length);

    for line in input.lines() {
        let v = line
            .split_whitespace()
            .map(str::parse)
            .filter_map(Result::ok)
            .collect::<Vec<u32>>();

        // populate 2 lists
        list_1.push(v[0]);
        list_2.push(v[1]);

        // save how many times each number appears in the right list
        *counters.entry(v[1]).or_insert(0) += 1;
    }

    // sort the lists
    list_1.sort_unstable();
    list_2.sort_unstable();

    // calculate the distance between the lists
    let p1 = list_1
        .iter()
        .zip(list_2.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u32>();

    // calculate the similarity score
    let p2 = list_1
        .iter()
        .fold(0, |acc, x| acc + x * counters.get(x).unwrap_or(&0));

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(solve(input), (11, 31));
    }
}
