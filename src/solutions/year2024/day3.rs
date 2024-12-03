pub fn part1(input: &str) -> impl std::fmt::Display {
    regex::Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [first, second])| first.parse::<u64>().unwrap() * second.parse::<u64>().unwrap())
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut enabled = true;
    regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(input)
        .filter_map(|caps| {
            if let Some(first) = caps.get(1) {
                if let Some(second) = caps.get(2) {
                    if enabled {
                        return Some(
                            first.as_str().parse::<u64>().unwrap()
                                * second.as_str().parse::<u64>().unwrap(),
                        );
                    }
                }
            } else if caps
                .get(0)
                .is_some_and(|m| m.as_str() == "do()" || m.as_str() == "don't()")
            {
                enabled = caps.get(0).unwrap().as_str() == "do()";
            }
            None
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1).to_string(), String::from("161"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("48"))
    }
}
