use atoi::atoi;
use bstr::ByteSlice;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .as_bytes()
        .lines()
        .map(|line| atoi::<u64>(line).unwrap() / 3 - 2)
        .sum::<u64>()
}

fn get_fuel(fuel: u64) -> u64 {
    let new_fuel = (fuel / 3).saturating_sub(2);
    if new_fuel == 0 {
        new_fuel
    } else {
        new_fuel + get_fuel(new_fuel)
    }
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .as_bytes()
        .lines()
        .map(|line| get_fuel(atoi::<u64>(line).unwrap()))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "100756";
    const INPUT2: &str = "100756";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT1).to_string(), String::from("33583"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2).to_string(), String::from("50346"))
    }
}
