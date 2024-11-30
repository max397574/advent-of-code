pub fn part1(input: &str) -> impl std::fmt::Display {
    let lines = input.split('\n');
    let mut increments = 0;
    let mut previous = 10000000;
    for line in lines {
        if !line.is_empty() {
            let val = line.parse().unwrap();
            if val > previous {
                increments += 1;
            }
            previous = val;
        }
    }
    increments
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut increments = 0;
    let mut previous = 65000;
    for line in input.split('\n').collect::<Vec<&str>>()[..].windows(3) {
        if !line.is_empty() {
            let val = line.iter().map(|f| f.parse::<u16>().unwrap()).sum();
            if val > previous {
                increments += 1;
            }
            previous = val;
        }
    }
    increments
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "199
200
208
210
200
207
240
269
260
263";
    const INPUT2: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1).to_string(), String::from("7"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("5"))
    }
}
