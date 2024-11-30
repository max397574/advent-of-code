pub fn part1(input: &str) -> impl std::fmt::Display {
    let lines = input.lines();
    let re = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut amount = 0;
    for line in lines {
        let captures = re.captures(line).unwrap();
        // TODO: rewrite like this https://github.com/orlp/aoc2022/blob/fd9b8157e5fdd1a9acc4d1df9dafefdc552fe42c/src/bin/day04.rs#L14
        // TODO: you could use captures[1] for example.
        if (captures.get(1).unwrap().as_str().parse::<i32>().unwrap()
            <= captures.get(3).unwrap().as_str().parse::<i32>().unwrap()
            && captures.get(2).unwrap().as_str().parse::<i32>().unwrap()
                >= captures.get(4).unwrap().as_str().parse::<i32>().unwrap())
            || (captures.get(3).unwrap().as_str().parse::<i32>().unwrap()
                <= captures.get(1).unwrap().as_str().parse::<i32>().unwrap()
                && captures.get(4).unwrap().as_str().parse::<i32>().unwrap()
                    >= captures.get(2).unwrap().as_str().parse::<i32>().unwrap())
        {
            amount += 1;
        }
    }
    amount
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let lines = input.lines();
    let re = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut amount = 0;
    for line in lines {
        let captures = re.captures(line).unwrap();
        if (captures.get(2).unwrap().as_str().parse::<i32>().unwrap()
            >= captures.get(3).unwrap().as_str().parse::<i32>().unwrap()
            && captures.get(1).unwrap().as_str().parse::<i32>().unwrap()
                <= captures.get(3).unwrap().as_str().parse::<i32>().unwrap())
            || (captures.get(4).unwrap().as_str().parse::<i32>().unwrap()
                >= captures.get(1).unwrap().as_str().parse::<i32>().unwrap()
                && captures.get(3).unwrap().as_str().parse::<i32>().unwrap()
                    <= captures.get(1).unwrap().as_str().parse::<i32>().unwrap())
        {
            amount += 1;
        }
    }
    amount
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("2"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("4"))
    }
}
