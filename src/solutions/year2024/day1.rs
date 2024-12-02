use bstr::ByteSlice;

use crate::utils::parsing::{swar_parsing, ByteParsing};

fn simple_parse_part1(input: &[u8]) -> u64 {
    let line_length = unsafe { input.find_byte(b'\n').unwrap_unchecked() + 1 };
    let lines = input.len() / line_length;
    let mut col1: Vec<u64> = vec![0; lines + 1];
    let mut col2: Vec<u64> = vec![0; lines + 1];
    let idx1 = unsafe { input.iter().position(|b| *b == b' ').unwrap_unchecked() };
    let idx2 = idx1 + 3;
    let idx3 = idx1 + idx2;
    input.chunks(line_length).enumerate().for_each(|(i, line)| {
        col1[i] = line[0..idx1].as_num::<u64>();
        col2[i] = line[idx2..idx3].as_num::<u64>();
    });
    col1.sort_unstable();
    col2.sort_unstable();
    col1.iter()
        .zip(col2)
        .map(|(val1, val2)| val1.abs_diff(val2))
        .sum::<u64>()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let line_length = unsafe { input.find_byte(b'\n').unwrap_unchecked() + 1 };
    if line_length != 14 {
        return simple_parse_part1(input);
    }
    let lines = input.len() / line_length;
    let mut col1: Vec<u64> = vec![0; lines];
    let mut col2: Vec<u64> = vec![0; lines];
    let idx1 = unsafe { input.iter().position(|b| *b == b' ').unwrap_unchecked() };
    let idx2 = idx1 + 3;
    let idx3 = idx1 + idx2;
    input.chunks(line_length).enumerate().for_each(|(i, line)| {
        col1[i] = swar_parsing(u64::from_be_bytes(line[0..idx2].try_into().unwrap()) >> 24);
        col2[i] =
            swar_parsing(u64::from_be_bytes(line[idx1..idx3].try_into().unwrap()) & 0xffffffffff);
    });
    col1.sort_unstable();
    col2.sort_unstable();
    col1.iter()
        .zip(col2)
        .map(|(val1, val2)| val1.abs_diff(val2))
        .sum::<u64>()
}

fn simple_parse_part2(input: &[u8]) -> usize {
    let line_length = unsafe { input.find_byte(b'\n').unwrap_unchecked() + 1 };
    let lines = input.len() / line_length;
    let mut col1: Vec<usize> = vec![0; lines + 1];
    let mut counts: [u16; 100000] = [0; 100000];
    let idx1 = unsafe { input.find_byte(b' ').unwrap_unchecked() };
    let idx2 = idx1 + 3;
    let idx3 = idx1 + idx2;
    input.chunks(line_length).enumerate().for_each(|(i, line)| {
        col1[i] = line[0..idx1].as_num::<usize>();
        counts[line[idx2..idx3].as_num::<usize>()] += 1;
    });

    col1.iter()
        .map(|num| num * counts[*num] as usize)
        .sum::<usize>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let line_length = unsafe { input.find_byte(b'\n').unwrap_unchecked() + 1 };
    if line_length != 14 {
        return simple_parse_part2(input);
    }
    let lines = input.len() / line_length;
    let mut col1: Vec<usize> = vec![0; lines];
    let mut counts: [u16; 100000] = [0; 100000];
    let idx1 = unsafe { input.find_byte(b' ').unwrap_unchecked() };
    let idx2 = idx1 + 3;
    let idx3 = idx1 + idx2;
    input.chunks(line_length).enumerate().for_each(|(i, line)| {
        col1[i] =
            swar_parsing(u64::from_be_bytes(line[0..idx2].try_into().unwrap()) >> 24) as usize;
        counts[swar_parsing(u64::from_be_bytes(line[idx1..idx3].try_into().unwrap()) & 0xffffffffff)
            as usize] += 1;
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
