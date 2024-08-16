pub fn part_1(input: &str) -> impl std::fmt::Display {
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

pub fn part_2(input: &str) -> impl std::fmt::Display {
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
                    output.push('.')
                }
                if cycles % 40 == 0 {
                    output.push('\n');
                }

                to_add = value.parse::<i32>().unwrap();
                cycles += 1;
                if ((cycles - 1) % 40 - register).abs() <= 1 {
                    output.push('#');
                } else {
                    output.push('.')
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
                    output.push('.')
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

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT).to_string(), String::from("13140"))
    }

    #[test]
    fn part2() {
        assert_eq!(
            part_2(INPUT).to_string(),
            String::from(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n"
            )
        )
    }
}
