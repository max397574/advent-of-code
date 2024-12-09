pub fn part1(_input: &str) -> impl std::fmt::Display {
    0
}

pub fn part2(_input: &str) -> impl std::fmt::Display {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("0"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(INPUT).to_string(), String::from("0"))
    }
}
