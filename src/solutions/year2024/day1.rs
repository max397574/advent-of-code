use bstr::ByteSlice;

use crate::utils::parsing::ByteParsing;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut col1: Vec<u16> = Vec::with_capacity(1000);
    let mut col2: Vec<u16> = Vec::with_capacity(1000);
    let idx1 = unsafe { input.iter().position(|b| *b == b' ').unwrap_unchecked() };
    let idx2 = idx1 + 3;
    let idx3 = idx1 + idx2;
    input.lines().for_each(|line| {
        col1.push(line[0..idx1].as_num());
        col2.push(line[idx2..idx3].as_num());
    });
    col1.sort_unstable();
    col2.sort_unstable();
    col1.iter()
        .zip(col2)
        .map(|(val1, val2)| val1.abs_diff(val2))
        .sum::<u16>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut col1: Vec<usize> = Vec::with_capacity(1000);
    let mut counts: [u16; 100000] = [0; 100000];
    let idx1 = unsafe { input.find_byte(b' ').unwrap_unchecked() };
    let idx2 = idx1 + 3;
    let idx3 = idx1 + idx2;
    input.lines().for_each(|line| {
        col1.push(line[0..idx1].as_num());
        counts[line[idx2..idx3].as_num::<usize>()] += 1;
    });

    col1.iter()
        .map(|num| num * counts[*num] as usize)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("11"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("31"))
    }
}
