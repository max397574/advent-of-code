use std::intrinsics::{unchecked_add, unchecked_div, unchecked_mul, unchecked_sub};

#[inline(always)]
fn get_multiple_sum(start: u64, end: u64, base: u64) -> u64 {
    unsafe {
        let first_multiple = unchecked_div(start + base - 1, base);
        let last_multiple = unchecked_div(end, base);
        // dbg!(first_multiple, last_multiple);

        base.unchecked_mul(
            (unchecked_mul(
                unchecked_add(first_multiple, last_multiple),
                unchecked_sub(last_multiple, first_multiple) + 1,
            )) / 2,
        )
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    unsafe {
        let mut solution: u64 = 0;
        let mut idx = 0;
        let input = input.as_bytes();
        while idx < input.len() {
            let start_start = idx;
            let mut start: u64 = 0;
            while *input.get_unchecked(idx) != b'-' {
                start = unchecked_add(
                    start.unchecked_mul(10_u64),
                    (input.get_unchecked(idx).unchecked_sub(b'0')) as u64,
                );
                idx = idx.unchecked_add(1);
            }
            let start_digits = idx.unchecked_sub(start_start);
            idx = idx.unchecked_add(1);

            let end_start = idx;
            let mut end: u64 = 0;
            // both , and \n are smaller than b'0'
            while *input.get_unchecked(idx) >= b'0' {
                end = unchecked_add(
                    end.unchecked_mul(10_u64),
                    (input.get_unchecked(idx).unchecked_sub(b'0')) as u64,
                );
                idx = idx.unchecked_add(1);
            }
            let end_digits = idx.unchecked_sub(end_start);
            idx = idx.unchecked_add(1);

            const BASE_LUT: [u64; 11] = [0, 0, 11, 0, 101, 0, 1001, 0, 10001, 0, 100001];
            const END_LUT: [u64; 11] = [
                0,
                0,
                100,
                0,
                10000,
                0,
                1000000,
                0,
                100000000,
                0,
                10000000000,
            ];
            const START_LUT: [u64; 11] = [0, 0, 10, 0, 1000, 0, 100000, 0, 10000000, 0, 1000000000];

            if !(start_digits & 1 == 1 && start_digits & 1 == end_digits & 1) {
                if end_digits == start_digits {
                    solution = solution.unchecked_add(get_multiple_sum(
                        start,
                        end,
                        *BASE_LUT.get_unchecked(start_digits),
                    ));
                } else if start_digits & 1 == 0 {
                    solution = solution.unchecked_add(get_multiple_sum(
                        start,
                        *END_LUT.get_unchecked(start_digits),
                        *BASE_LUT.get_unchecked(start_digits),
                    ));
                } else {
                    solution = solution.unchecked_add(get_multiple_sum(
                        *START_LUT.get_unchecked(start_digits + 1),
                        end,
                        *BASE_LUT.get_unchecked(start_digits + 1),
                    ));
                }
            }
        }
        solution
    }
}

fn all_chunks_equal<T: PartialEq>(iter: impl Iterator<Item = T>, k: usize) -> bool {
    let vec: Vec<T> = iter.collect();
    let mut chunks = vec.chunks_exact(k);
    if let Some(first_chunk) = chunks.next() {
        chunks.all(|chunk| chunk == first_chunk)
    } else {
        unreachable!()
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start..=end)
                .filter(|num| {
                    let word = num.to_string();
                    (1..(word.len() / 2 + 1))
                        .any(|k| word.len().is_multiple_of(k) && all_chunks_equal(word.bytes(), k))
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("1227775554"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("4174379265"))
    }
}
