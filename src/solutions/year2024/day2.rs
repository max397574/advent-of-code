use bstr::ByteSlice;

use crate::utils::parsing::ByteParsing;

enum Behavior {
    Increasing,
    Decreasing,
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .split(|c| *c == b' ')
                .map(|chunk| chunk.as_num())
                .collect();
            let behavior = if numbers[0] < numbers[1] {
                Behavior::Increasing
            } else {
                Behavior::Decreasing
            };
            if check_if_valid(numbers, behavior) {
                1
            } else {
                0
            }
        })
        .sum::<u32>()
}

fn check_if_valid(numbers: Vec<u32>, behavior: Behavior) -> bool {
    let mut i = 0;
    while i < numbers.len() - 1 {
        if let Behavior::Increasing = behavior {
            if numbers[i + 1] <= numbers[i] || numbers[i + 1] - numbers[i] > 3 {
                return false;
            }
        } else if numbers[i + 1] >= numbers[i] || numbers[i] - numbers[i + 1] > 3 {
            return false;
        }
        i += 1;
    }
    true
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .split(|c| *c == b' ')
                .map(|chunk| chunk.as_num())
                .collect();
            let mut j = 0;
            let mut any_true = false;
            while j < numbers.len() {
                let mut nums = numbers.clone();
                nums.remove(j);
                let behavior = if nums[0] < nums[1] {
                    Behavior::Increasing
                } else {
                    Behavior::Decreasing
                };
                if check_if_valid(nums, behavior) {
                    any_true = true;
                }
                if any_true {
                    return 1;
                }
                j += 1;
            }
            let behavior = if numbers[0] < numbers[1] {
                Behavior::Increasing
            } else {
                Behavior::Decreasing
            };
            if check_if_valid(numbers, behavior) {
                1
            } else {
                0
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_() {
        assert_eq!(part1(INPUT).to_string(), String::from("2"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("4"))
    }
}
