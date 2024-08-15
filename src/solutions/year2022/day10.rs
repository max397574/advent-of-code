pub fn part_1(_input: &str) -> impl std::fmt::Display {
    let mut result = 0;
    let mut register = 1;
    let mut cycles = 1;
    let mut to_add;
    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["addx", value] => {
                if cycles % 40 == 20 || cycles == 20 {
                    result += cycles * register;
                }
                to_add = value.parse::<i32>().unwrap();
                cycles += 1;
                if cycles % 40 == 20 || cycles == 20 {
                    result += cycles * register;
                }
                cycles += 1;
                register += to_add;
            }
            ["noop"] => {
                if cycles % 40 == 20 || cycles == 20 {
                    result += cycles * register;
                }
                cycles += 1;
            }
            _ => {
                unreachable!()
            }
        }
    }
    result
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    let mut register: i32 = 1;
    let mut cycles: i32 = 1;
    let mut to_add;
    let mut output = String::with_capacity(40 * 7);
    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["addx", value] => {
                if ((cycles - 1) % 40 - register).abs() <= 1 {
                    output.push('#');
                } else {
                    output.push(' ')
                }
                if cycles % 40 == 0 {
                    output.push('\n');
                }

                to_add = value.parse::<i32>().unwrap();
                cycles += 1;
                if ((cycles - 1) % 40 - register).abs() <= 1 {
                    output.push('#');
                } else {
                    output.push(' ')
                }
                if cycles % 40 == 0 {
                    output.push('\n');
                }

                cycles += 1;
                register += to_add;
            }
            ["noop"] => {
                if ((cycles - 1) % 40 - register).abs() <= 1 {
                    output.push('#');
                } else {
                    output.push(' ')
                }
                if cycles % 40 == 0 {
                    output.push('\n');
                }
                cycles += 1;
            }
            _ => {
                unreachable!()
            }
        }
    }
    output
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
