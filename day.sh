#!/bin/bash

session=$(<".session")

day=$1
year=2024
# curl --cookie "session=$session" "https://adventofcode.com/$year/day/$1/input" >> input/day$1.txt
echo "pub fn part_1(_input: &str) -> impl std::fmt::Display {
    0
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    0
}

// #[cfg(test)]
mod tests {
    use super::*;
    const _INPUT: &str = \"\";

    // #[test]
    fn _part1() {
        assert_eq!(part_1(_INPUT).to_string(), String::from(\"0\"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part_2(_INPUT).to_string(), String::from(\"0\"))
    }
}" > "./src/solutions/year$year/day$day.rs"
