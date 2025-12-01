use bstr::ByteSlice;

use crate::utils::parsing::ByteParsing;

/// L: 01001100 -> -1
/// R: 01010010 -> 1
#[inline(always)]
fn lr_hash(byte: u8) -> i32 {
    1 - 2 * (((byte & 4) >> 2) as i32)
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let mut current = 50;
    let mut zero_count = 0;
    input.as_bytes().lines().for_each(|line| {
        let delta = lr_hash(line[0]) * line[1..].as_num::<i32>();
        current += delta;
        current = current.rem_euclid(100);
        if current == 0 {
            zero_count += 1;
        }
    });
    zero_count
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let mut current = 50;
    let mut zero_count = 0;
    input.as_bytes().lines().for_each(|line| {
        let delta = lr_hash(line[0]) * line[1..].as_num::<i32>();
        let new_tmp = current + (delta % 100);
        current += delta;
        current = current.rem_euclid(100);
        zero_count += delta / 100;
        if current == 0 {
            zero_count += 1;
        }
        if !(0..=100).contains(&new_tmp) {
            zero_count += 1;
        }
    });
    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn hash_function() {
        assert_eq!(lr_hash(b'L'), -1);
        assert_eq!(lr_hash(b'R'), 1);
    }

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("3"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("6"))
    }
}
