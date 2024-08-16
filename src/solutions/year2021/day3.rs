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
    let width = input.lines().next().unwrap().len();
    let nums = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
    let oxy = (0..width)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..width)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.drain_filter(|n| (*n & 1 << i > 0) == one);
            co2.first().copied()
        })
        .last()
        .unwrap();
    co2 * oxy
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
