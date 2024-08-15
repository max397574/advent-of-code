use atoi::atoi;
use bstr::ByteSlice;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut score = 0;
    let input = input.as_bytes();
    let numbers_per_line = ((input.find_byte(b'\n').unwrap() - 9) / 3) as u32;
    for line in input.lines() {
        let mut numbers: u128 = 0;
        for block in line[9..].split(|&byte| byte == b' ') {
            if block.is_empty() {
                continue;
            }
            if block[block.len() - 1] == b':' || block[0] == b'|' {
                continue;
            }

            numbers |= 1 << atoi::<u8>(block).unwrap();
        }
        // HACK: total amount of numbers - count ones will give the amount of
        // duplicated because there are no duplicates in input
        score += (1 << (numbers_per_line - numbers.count_ones())) >> 1;
    }
    score
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut matching_numbers = Vec::new();
    let input = input.as_bytes();
    let numbers_per_line = ((input.find_byte(b'\n').unwrap() - 9) / 3) as u32;
    for (idx, line) in input.lines().enumerate() {
        let mut numbers: u128 = 0;
        for block in line.split(|&byte| byte == b' ') {
            if block.is_empty() {
                continue;
            }
            if block[0] == b'C' {
                continue;
            }
            if block[block.len() - 1] == b':' || block[0] == b'|' {
                continue;
            }

            numbers |= 1 << atoi::<u8>(block).unwrap();
        }
        // HACK: total amount of numbers - count ones will give the amount of
        // duplicated because there are no duplicates in input
        println!("{idx}");
        matching_numbers.push(numbers_per_line - numbers.count_ones());
    }

    let mut copies = vec![1; matching_numbers.len()];
    let mut count = 0;
    for (idx, match_count) in matching_numbers.iter().enumerate() {
        count += copies[idx];
        for i in 1..=*match_count {
            copies[idx + i as usize] += copies[idx];
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const INPUT2: &str = INPUT1;

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT1).to_string(), String::from("13"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2).to_string(), String::from("30"))
    }
}
