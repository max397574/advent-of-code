use bstr::ByteSlice;

use crate::utils::parsing::ByteParsing;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    fn is_possible(res: u64, values: &[u64]) -> bool {
        let last_value = values[values.len() - 1];
        if values.len() == 1 {
            res == last_value
        } else {
            (last_value < res && is_possible(res - last_value, &values[..values.len() - 1]))
                || (res.is_multiple_of(last_value)
                    && is_possible(res / last_value, &values[..values.len() - 1]))
        }
    }

    let input = input.as_bytes();
    input
        .lines()
        .filter_map(|line| {
            let idx = line.find_byte(b':').unwrap();
            let num: u64 = line[0..idx].as_num();
            let components: Vec<u64> = line[idx + 2..]
                .split(|c| *c == b' ')
                .map(|val| val.as_num())
                .collect();

            if is_possible(num, &components[..]) {
                Some(num)
            } else {
                None
            }
        })
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    fn is_possible(res: u64, values: &[u64]) -> bool {
        let last_value = values[values.len() - 1];
        if values.len() == 1 {
            res == last_value
        } else {
            // return if no of the cases is possible
            (last_value < res && is_possible(res - last_value, &values[..values.len() - 1]))
                || (res.is_multiple_of(last_value)
                    && is_possible(res / last_value, &values[..values.len() - 1]))
                || (res % 10_u64.pow(last_value.ilog10() + 1) == last_value
                    && is_possible(
                        res / 10_u64.pow(last_value.ilog10() + 1),
                        &values[..values.len() - 1],
                    ))
        }
    }

    let input = input.as_bytes();
    input
        .lines()
        .filter_map(|line| {
            let idx = line.find_byte(b':').unwrap();
            let num: u64 = line[0..idx].as_num();
            let components: Vec<u64> = line[idx + 2..]
                .split(|c| *c == b' ')
                .map(|val| val.as_num())
                .collect();

            if is_possible(num, &components[..]) {
                Some(num)
            } else {
                None
            }
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("3749"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("11387"))
    }
}
