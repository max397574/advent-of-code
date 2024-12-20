use std::collections::HashMap;

fn is_possible(towel: &str, patterns: &Vec<&str>, cache: &mut HashMap<String, bool>) -> bool {
    if towel.is_empty() {
        return true;
    }
    if cache.contains_key(towel) {
        return *cache.get(towel).unwrap();
    }
    for pattern in patterns {
        if let Some(rest) = towel.strip_suffix(pattern) {
            if is_possible(rest, patterns, cache) {
                cache.insert(rest.to_owned(), true);
                return true;
            } else {
                cache.insert(rest.to_owned(), false);
            }
        }
    }
    false
}

pub fn part1(input: &str) -> usize {
    let (patterns, towels) = input.trim().split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns.split(", ").collect();
    let towels: Vec<&str> = towels.lines().collect();
    let mut cache: HashMap<String, bool> = HashMap::new();
    towels
        .iter()
        .filter(|towel| is_possible(towel, &patterns, &mut cache))
        .count()
}

#[inline(always)]
fn count_combinations(towel: &str, patterns: &Vec<&str>, cache: &mut HashMap<String, u64>) -> u64 {
    if towel.is_empty() {
        return 1;
    }
    if cache.contains_key(towel) {
        return *cache.get(towel).unwrap();
    }
    let mut count = 0;
    for pattern in patterns {
        if let Some(rest) = towel.strip_prefix(pattern) {
            count += count_combinations(rest, patterns, cache);
        }
    }
    cache.insert(towel.to_owned(), count);
    count
}

pub fn part2(input: &str) -> u64 {
    let (patterns, towels) = input.trim().split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns.split(", ").collect();
    let towels: Vec<&str> = towels.lines().collect();
    let mut cache: HashMap<String, u64> = HashMap::new();
    towels
        .iter()
        .map(|towel| count_combinations(towel, &patterns, &mut cache))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("6"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("16"))
    }
}
