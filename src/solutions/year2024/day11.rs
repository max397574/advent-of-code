#![allow(long_running_const_eval)]
use std::collections::HashMap;

use crate::utils::parsing::ByteParsing;

pub fn part1_slower(input: &str) -> u64 {
    let mut nums: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|num| (num.as_bytes()).as_num())
        .collect();
    (0..25).for_each(|_| {
        let mut new = Vec::new();
        let len = nums.len();
        for i in 0..len {
            let num = nums[i];
            if num == 0 {
                new.push(1);
            } else if (num.ilog10() + 1) % 2 == 0 {
                let first = num % 10_u64.pow((num.ilog10() + 1) / 2);
                let second = num / 10_u64.pow((num.ilog10() + 1) / 2);
                nums[i] = first;
                nums.push(second);
            } else {
                new.push(num * 2024);
            }
        }
    });
    nums.len() as u64
}

#[inline(always)]
pub fn fast_solution(input: &str, runs: u8) -> u64 {
    let nums: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|num| (num.as_bytes()).as_num())
        .collect();

    fn rec(num: u64, iteration: u8) -> u64 {
        if num < 1000 {
            return CACHE[iteration as usize][num as usize];
        }
        if iteration == 0 {
            1
        } else if num == 0 {
            rec(1, iteration - 1)
        } else if (num.ilog10() + 1) % 2 == 0 {
            let first = num % 10_u64.pow((num.ilog10() + 1) / 2);
            let second = num / 10_u64.pow((num.ilog10() + 1) / 2);
            rec(first, iteration - 1) + rec(second, iteration - 1)
        } else {
            rec(num * 2024, iteration - 1)
        }
    }

    nums.iter().map(|&num| rec(num, runs)).sum::<u64>()
}

pub fn part2_slower(input: &str) -> u64 {
    let nums: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|num| (num.as_bytes()).as_num())
        .collect();

    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();

    fn rec(num: u64, iteration: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
        if let Some(&val) = cache.get(&(num, iteration)) {
            return val;
        }
        let result = if iteration == 0 {
            1
        } else if num == 0 {
            rec(1, iteration - 1, cache)
        } else if (num.ilog10() + 1) % 2 == 0 {
            let first = num % 10_u64.pow((num.ilog10() + 1) / 2);
            let second = num / 10_u64.pow((num.ilog10() + 1) / 2);
            rec(first, iteration - 1, cache) + rec(second, iteration - 1, cache)
        } else {
            rec(num * 2024, iteration - 1, cache)
        };
        cache.insert((num, iteration), result);
        result
    }

    nums.iter()
        .map(|&num| rec(num, 75, &mut cache))
        .sum::<u64>()
}

const fn calc_cache(iteration: usize, num: usize, lut: &[[u64; 1000]; 76]) -> u64 {
    if iteration == 0 {
        1
    } else if num < 1000 {
        lut[iteration][num]
    } else if num == 0 {
        lut[iteration - 1][1]
    } else if num.ilog10() % 2 == 1 {
        let pow10 = 10usize.pow((num.ilog10() + 1) / 2);
        calc_cache(iteration - 1, num / pow10, lut) + calc_cache(iteration - 1, num % pow10, lut)
    } else {
        calc_cache(iteration - 1, num * 2024, lut)
    }
}

const CACHE_SIZE: usize = 1000;
const CACHE_DEPTH: usize = 76;
const CACHE: [[u64; CACHE_SIZE]; CACHE_DEPTH] = {
    let mut cache = [[0; CACHE_SIZE]; CACHE_DEPTH];
    let mut num = 0;
    while num < 1000 {
        cache[0][num] = 1;
        num += 1;
    }
    let mut iteration = 1;
    while iteration < CACHE_DEPTH {
        let mut num = 0;
        while num < CACHE_SIZE {
            let val = if num == 0 {
                cache[iteration - 1][1]
            } else if num.ilog10() % 2 == 1 {
                let pow10 = 10usize.pow((num.ilog10() + 1) / 2);
                cache[iteration - 1][num / pow10] + cache[iteration - 1][num % pow10]
            } else {
                calc_cache(iteration - 1, num * 2024, &cache)
            };

            cache[iteration][num] = val;
            num += 1;
        }
        iteration += 1;
    }
    cache
};

pub fn part2(input: &str) -> u64 {
    //part2_slower(input)
    fast_solution(input, 75)
}

pub fn part1(input: &str) -> u64 {
    //part1_slower(input)
    fast_solution(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "125 17";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("55312"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(INPUT).to_string(), String::from(""))
    }
}
