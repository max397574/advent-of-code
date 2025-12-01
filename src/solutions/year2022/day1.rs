pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    input
        .split("\n\n")
        .map(|set| {
            set.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let mut sorted = input
        .split("\n\n")
        .map(|set| {
            set.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    sorted.sort();
    let length = sorted.len();
    sorted.into_iter().skip(length - 3).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("24000"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("45000"))
    }
}
