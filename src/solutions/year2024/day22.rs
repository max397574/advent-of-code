pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut num = line.parse::<u64>().unwrap();
            for _ in 0..2000 {
                num ^= num << 6;
                num %= 16777216;

                num ^= num >> 5;
                num %= 16777216;

                num ^= num << 11;
                num %= 16777216;
            }
            num
        })
        .sum()
}

pub fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1
10
100
2024";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("37327623"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("23"))
    }
}
