use bstr::ByteSlice;

use crate::utils::parsing::ByteParsing;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let input = input.as_bytes();
    input
        .trim()
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut numbers = line.split(|c| *c == b' ').map(|chunk| chunk.as_num());
            let first: u32 = unsafe { numbers.next().unwrap_unchecked() };
            let mut second: u32 = unsafe { numbers.next().unwrap_unchecked() };
            if !matches!(first.abs_diff(second), 1..=3) {
                return 0;
            }
            let direction = first.cmp(&second);
            numbers.all(|num| {
                let val = (matches!(second.abs_diff(num), 1..=3) && second.cmp(&num) == direction);
                second = num;
                val
            }) as u32
        })
        .sum::<u32>()
}

fn check_line(numbers: &[u32]) -> bool {
    let mut numbers = numbers.iter().copied();
    let first: u32 = unsafe { numbers.next().unwrap_unchecked() };
    let mut second: u32 = unsafe { numbers.next().unwrap_unchecked() };
    if !matches!(first.abs_diff(second), 1..=3) {
        return false;
    }
    let direction = first.cmp(&second);
    numbers.all(|num| {
        let val = (matches!(second.abs_diff(num), 1..=3) && second.cmp(&num) == direction);
        second = num;
        val
    })
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let input = input.as_bytes();
    input
        .lines()
        .map(|line| {
            let mut numbers: Vec<u32> = line
                .split(|c| *c == b' ')
                .map(|chunk| chunk.as_num())
                .collect();
            let mut num_it = numbers.iter().copied();
            let first: u32 = unsafe { num_it.next().unwrap_unchecked() };
            let mut second: u32 = unsafe { num_it.next().unwrap_unchecked() };
            let direction = first.cmp(&second);
            let mut checked: usize = 1;
            if matches!(first.abs_diff(second), 1..=3)
                && num_it.all(|num| {
                    let val =
                        (matches!(second.abs_diff(num), 1..=3) && second.cmp(&num) == direction);
                    second = num;
                    if val {
                        checked += 1;
                    }
                    val
                })
            {
                return 1;
            }
            let len = numbers.len();
            for j in checked.saturating_sub(1)..=checked + 1 {
                let old = numbers[j];
                numbers.copy_within(j + 1.., j);
                if check_line(&numbers[..len - 1]) {
                    return 1;
                }
                numbers.copy_within(j..len - 2, j + 1);
                numbers[j] = old;
            }
            0
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
