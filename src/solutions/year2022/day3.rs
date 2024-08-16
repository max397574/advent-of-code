pub fn part_1(input: &str) -> impl std::fmt::Display {
    let lines = input.lines();
    let mut score = 0;
    for line in lines {
        let mut checked: Vec<char> = vec![];
        let length = line.len();
        let first = &line[..length / 2];
        let second = &line[length / 2..];
        for item in first.chars() {
            if second.contains(item) {
                if checked.contains(&item) {
                    break;
                }
                checked.push(item);
                if item.is_lowercase() {
                    score += (item as usize) - 96;
                } else {
                    score += (item as usize) - 38;
                }
            }
        }
    }
    score
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let lines = input.lines();
    let mut score = 0;
    for line in lines.array_chunks::<3>() {
        for item in line[0].chars() {
            if line[1].contains(item) && line[2].contains(item) {
                if item.is_lowercase() {
                    score += (item as usize) - 96;
                } else {
                    score += (item as usize) - 38;
                }
                break;
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT).to_string(), String::from("157"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT).to_string(), String::from("70"))
    }
}
