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
    let mut current = 0;
    for (i, char) in input.bytes().enumerate() {
        current += match char {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        };
        if current == -1 {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "(()";
    const INPUT2: &str = "()())(";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1).to_string(), String::from("1"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("5"))
    }
}
