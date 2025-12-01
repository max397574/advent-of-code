pub fn part1(_input: &str) -> impl std::fmt::Display + use<> {
    0
}

pub fn part2(_input: &str) -> impl std::fmt::Display + use<> {
    0
}

// #[cfg(test)]
mod tests {
    use super::*;
    const _INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    const _INPUT2: &str = "";

    // #[test]
    fn _part_1() {
        assert_eq!(part1(_INPUT).to_string(), String::from("5"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(_INPUT2).to_string(), String::from("0"))
    }
}
