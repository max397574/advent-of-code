pub fn part_1(input: &str) -> impl std::fmt::Display {
    0
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    const INPUT2: &str = "";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT1).to_string(), String::from("0"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2).to_string(), String::from("0"))
    }
}
