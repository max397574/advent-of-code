pub fn part_1(_input: &str) -> impl std::fmt::Display {
    let lines = input.lines();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
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

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    let lines = input.lines();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
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

// #[cfg(test)]
mod tests {
    use super::*;
    const _INPUT1: &str = "";
    const _INPUT2: &str = "";

    // #[test]
    fn _part1() {
        assert_eq!(part_1(_INPUT1).to_string(), String::from("0"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part_2(_INPUT2).to_string(), String::from("0"))
    }
}
