fn parse(input: &str) -> Vec<usize> {
    let mut values = Vec::with_capacity(600);
    let input = input.as_bytes();
    for line in input.split(|&c| c == b'\n') {
        let mut val = 0;
        for byte in line {
            val = val * 10 + (byte - b'0') as usize;
        }
        values.push(val);
    }
    values
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut hash = [false; 2021];
    for line in input.split(|&c| c == b'\n') {
        let mut val = 0;
        for byte in line {
            val = val * 10 + (byte - b'0') as usize;
        }

        if hash[val] {
            return val * (2020 - val);
        } else {
            hash[2020 - val] = true;
        }
    }
    0
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    part2_inner(&parse(input))
}

pub fn part2_inner(values: &[usize]) -> impl std::fmt::Display {
    let mut hash = [0; 2021];
    for j in 0..values.len() {
        let i = values[j];
        for val in values {
            if hash[*val] == j + 1 {
                return val * (2020 - i - val) * i;
            } else if val + i < 2020 {
                hash[2020 - i - val] = j + 1;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1721
979
366
299
675
1456";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("514579"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("241861950"))
    }
}
