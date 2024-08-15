pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut bit_1: Vec<usize> = vec![0; input.lines().next().unwrap().len()];
    let mut total = 0;
    for line in input.lines() {
        let bytes = line.bytes();
        for (index, byte) in bytes.enumerate() {
            if byte as char == '1' {
                bit_1[index] += 1;
            }
        }
        total += 1;
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for bit in bit_1.iter() {
        if bit > &(total / 2) {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma = usize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon = usize::from_str_radix(epsilon.as_str(), 2).unwrap();
    gamma * epsilon
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "";
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
