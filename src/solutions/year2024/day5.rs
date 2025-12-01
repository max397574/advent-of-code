use std::collections::HashSet;

use bstr::ByteSlice;

use crate::utils::parsing::ByteParsing;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.as_bytes();
    let updates = updates.as_bytes();

    let rules: HashSet<(u32, u32)> = rules
        .lines()
        .map(|rule| (rule[0..2].as_num(), rule[3..].as_num()))
        .collect();

    let updates: Vec<Vec<u32>> = updates
        .lines()
        .map(|update| {
            update
                .split(|c| *c == b',')
                .map(|num| num.as_num())
                .collect()
        })
        .collect();

    updates
        .iter()
        .filter(|update| update.is_sorted_by(|&l, &r| rules.contains(&(l, r))))
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.as_bytes();
    let updates = updates.as_bytes();

    let rules: HashSet<(u32, u32)> = rules
        .lines()
        .map(|rule| (rule[0..2].as_num(), rule[3..].as_num()))
        .collect();

    let updates: Vec<Vec<u32>> = updates
        .lines()
        .map(|update| {
            update
                .split(|c| *c == b',')
                .map(|num| num.as_num())
                .collect()
        })
        .collect();

    updates
        .iter()
        .filter(|update| !update.is_sorted_by(|&l, &r| rules.contains(&(l, r))))
        .map(|update| {
            let middle_idx = update.len() / 2;
            let mut update = update.clone();
            let (_, &mut val, _) = update.select_nth_unstable_by(middle_idx, |&a, &b| {
                if rules.contains(&(a, b)) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            val
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "47|53
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
97,13,75,29,47";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("143"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("123"))
    }
}
