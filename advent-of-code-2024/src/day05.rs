// https://adventofcode.com/2024/day/5

use std::vec;

pub fn solve(input: &str) -> (u32, u32) {
    let mut rules = std::collections::BTreeMap::new();

    let mut it = input.lines(); // this allow to consume iterator in 2 phases

    for line in &mut it {
        if line.is_empty() {
            break;
        }

        let v = line.split('|').collect::<Vec<_>>();
        let a = v[0].parse::<u32>().unwrap();
        let b = v[1].parse::<u32>().unwrap();

        rules
            .entry(a)
            .and_modify(|e: &mut Vec<u32>| e.push(b))
            .or_insert_with(|| vec![b]);
    }

    let mut sum_of_middle_values = 0;

    for line in &mut it {
        let v = line.split(',').collect::<Vec<_>>();
        let v = v
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut ok = true;

        for (idx, value) in v.iter().enumerate() {
            let r = rules.get(value);

            // no rule for this number, so it's ok
            if r.is_none() {
                continue;
            }

            // let r = r.unwrap();
            // println!("{value} {r:?}, {set:?}");

            for rr in r.unwrap() {
                // if the number if the rule is already before in the list, it's not ok
                if v[0..idx].contains(rr) {
                    ok = false;
                    break;
                }
            }

            // if it's not ok, we can break the loop without checking the rest of the list
            if !ok {
                break;
            }
        }

        if ok {
            sum_of_middle_values += v[v.len() / 2];
        }
    }

    (sum_of_middle_values, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        assert_eq!(solve(input), (143, 0));
    }
}
