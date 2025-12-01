use bstr::ByteSlice;
use std::fmt::Display;

pub fn part1(input: &str) -> impl Display + use<> {
    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    const OFFSET_PER_LINE: u32 = 11 * b'0' as u32;
    let mut line_count = 0;
    for line in input.as_bytes().lines() {
        line_count += 1;
        let mut line_iter = line.iter();
        let first = line_iter.find(|c| *c <= &b'9').unwrap();
        sum2 += *line_iter.rfind(|c| *c <= &b'9').unwrap_or(first) as u32;
        sum1 += *first as u32;
    }
    sum1 * 10 + sum2 - line_count * OFFSET_PER_LINE
}
pub fn part2(input: &str) -> impl Display + use<> {
    const OPTIONS: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];
    input
        .as_bytes()
        .lines()
        .map(|line| {
            let first: u8 = line
                .iter()
                .enumerate()
                .find_map(|(idx, &char)| {
                    if char <= b'9' {
                        return Some(char - b'0');
                    }

                    if char > b'f' && char < b'n' {
                        return None;
                    }

                    for (index, option) in OPTIONS.iter().enumerate() {
                        if char == option[0] && line[idx..].starts_with(option) {
                            return Some((index + 1) as u8);
                        }
                    }
                    None
                })
                .unwrap();
            let last: u8 = line
                .iter()
                .rev()
                .enumerate()
                .find_map(|(idx, &char)| {
                    if char <= b'9' {
                        return Some(char - b'0');
                    }

                    if (char > b'e' && char < b'n') || (char < b'e' && char > b'9') {
                        return None;
                    }

                    for (index, option) in OPTIONS.iter().enumerate() {
                        if char == option[option.len() - 1]
                            && line[..(line.len() - idx)].ends_with(option)
                        {
                            return Some((index + 1) as u8);
                        }
                    }
                    None
                })
                .unwrap();
            (first * 10 + last) as u32
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("142"))
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("281"))
    }
}
