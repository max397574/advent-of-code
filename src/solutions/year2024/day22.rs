use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut num = line.parse::<u64>().unwrap();
            for _ in 0..2000 {
                num ^= num << 6;
                num %= 16777216;

                num ^= num >> 5;
                num %= 16777216;

                num ^= num << 11;
                num %= 16777216;
            }
            num
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let values: Vec<_> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut all_combinations = HashMap::new();
    for val in values {
        let mut seqs: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        let mut seq = (0, 0, 0, 0);
        let mut n = val;
        for i in 0..2000 {
            let prev = n % 10;
            n = (n ^ (n << 6)) % 16777216;
            n = (n ^ (n >> 5)) % 16777216;
            n = (n ^ (n << 11)) % 16777216;
            seq = (seq.1, seq.2, seq.3, n % 10 - prev);
            if i >= 3 && !seqs.keys().any(|&k| k == seq) {
                seqs.insert(seq, n % 10);
            }
        }
        for (k, v) in seqs {
            *all_combinations.entry(k).or_insert(0) += v;
        }
    }
    *all_combinations.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "1
10
100
2024";

    const INPUT2: &str = "1
2
3
2024";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1).to_string(), String::from("37327623"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("23"))
    }
}
