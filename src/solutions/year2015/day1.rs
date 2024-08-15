pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .bytes()
        .map(|char| match char {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum::<i32>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
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
    fn part1() {
        assert_eq!(part_1(_INPUT1).to_string(), String::from("0"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(_INPUT2).to_string(), String::from("5"))
    }
}
