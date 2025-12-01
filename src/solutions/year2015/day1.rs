pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    input
        .bytes()
        .map(|char| match char {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum::<i32>()
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    input
        .bytes()
        .map(|char| match char {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const _INPUT1: &str = "(()";
    const _INPUT2: &str = "()())(";

    #[test]
    fn part_1() {
        assert_eq!(part1(_INPUT1).to_string(), String::from("1"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(_INPUT2).to_string(), String::from("0"))
    }
}
