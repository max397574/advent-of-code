#![allow(unused_attributes)]
#![feature(portable_simd)]

use std::simd::prelude::*;

static mut KEYS: [u64; 250] = [0; 250];
static mut LOCKS: [u64; 250] = [0; 250];

pub fn run(input: &str) -> u64 {
    part1(input)
}

pub fn part1(input: &str) -> u64 {
    unsafe {
        let mut input = input.as_bytes().as_ptr();
        let mut count = 0;
        let mut block;
        let mut key_count = 0;
        let mut lock_count = 0;
        for _ in 0..500 {
            block = input.cast::<u8x64>().read_unaligned();
            // bitmask where # are
            let mut mask = block.simd_eq(std::simd::u8x64::splat(b'#')).to_bitmask();
            // just look at the first 42 bytes (avoid reading next input)
            mask &= (1 << 43) - 1;
            if mask & 1 == 0 {
                // is a lock
                *LOCKS.get_unchecked_mut(lock_count) = mask;
                lock_count += 1;
            } else {
                // is a key
                *KEYS.get_unchecked_mut(key_count) = mask;
                key_count += 1;
            }
            input = input.add(43);
        }

        for l in 0..250 {
            for k in 0..250 {
                if KEYS.get_unchecked(k) & LOCKS.get_unchecked(l) == 0 {
                    count += 1;
                }
            }
        }

        count
    }
}

pub fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("3"))
    }
}
