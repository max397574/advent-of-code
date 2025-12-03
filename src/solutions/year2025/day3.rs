pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let (pos, max) = find_max(line, 0, line.len() - 1);
            let (_, max2) = find_max(line, pos + 1, line.len());
            max * 10 + max2
        })
        .sum::<u64>()
}

fn find_max(line: &[u8], start: usize, end: usize) -> (usize, u64) {
    let mut max = 0;
    let mut max_pos = 0;
    for (i, byte) in line.iter().enumerate().take(end).skip(start) {
        if *byte > max {
            max = *byte;
            max_pos = i;
        }
    }
    (max_pos, (max - b'0') as u64)
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let mut start_pos = 0;
            let mut res = 0;
            for i in 1..=12 {
                let (pos, max) = find_max(line, start_pos, line.len() - 12 + i);
                start_pos = pos + 1;
                res = res * 10 + max;
            }
            res
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("357"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("3121910778619"))
    }
}
