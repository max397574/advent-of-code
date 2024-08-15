pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut depth = 0;
    let mut horizontal = 0;
    for line in input.lines() {
        let [cmd, amount] = line.split_whitespace().collect::<Vec<_>>()[..] else {
            unreachable!()
        };
        let amount = amount.parse::<usize>().unwrap();
        match cmd {
            "forward" => {
                horizontal += amount;
            }
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => {
                unreachable!()
            }
        }
    }
    depth * horizontal
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for line in input.lines() {
        let [cmd, amount] = line.split_whitespace().collect::<Vec<_>>()[..] else {
            unreachable!()
        };
        let amount = amount.parse::<usize>().unwrap();
        match cmd {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => {
                unreachable!()
            }
        }
    }
    depth * horizontal
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT).to_string(), String::from("150"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT).to_string(), String::from("900"))
    }
}
