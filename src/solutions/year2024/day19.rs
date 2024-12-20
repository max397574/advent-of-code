use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_leaf: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_leaf: false,
        }
    }

    fn insert(&mut self, word: &str) {
        if word.is_empty() {
            self.is_leaf = true;
            return;
        }
        let first = word.chars().next().unwrap();
        if let Some(subtree) = self.children.get_mut(&first) {
            subtree.insert(&word[1..]);
        } else {
            let mut subtree = Self::new();
            subtree.insert(&word[1..]);
            self.children.insert(first, subtree);
        }
    }

    fn is_possible(&self, towel: &str, cache: &mut HashMap<String, bool>) -> bool {
        if towel.is_empty() {
            return true;
        }

        if let Some(&result) = cache.get(towel) {
            return result;
        }

        let mut current = self;
        for (i, ch) in towel.char_indices() {
            if let Some(next) = current.children.get(&ch) {
                current = next;
                if current.is_leaf && self.is_possible(&towel[i + 1..], cache) {
                    cache.insert(towel.to_string(), true);
                    return true;
                }
            } else {
                cache.insert(towel.to_string(), false);
                return false;
            }
        }
        cache.insert(towel.to_string(), false);
        false
    }

    fn count_combinations(&self, towel: &str, cache: &mut HashMap<String, u64>) -> u64 {
        if towel.is_empty() {
            return 1;
        }

        if let Some(&count) = cache.get(towel) {
            return count;
        }

        let mut count = 0;
        let mut current = self;
        for (i, ch) in towel.char_indices() {
            if let Some(next) = current.children.get(&ch) {
                current = next;
                if current.is_leaf {
                    count += self.count_combinations(&towel[i + 1..], cache);
                }
            } else {
                break;
            }
        }
        cache.insert(towel.to_string(), count);
        count
    }
}

pub fn part1(input: &str) -> usize {
    let (patterns, towels) = input.trim().split_once("\n\n").unwrap();
    let mut trie = TrieNode::new();
    for pattern in patterns.split(", ") {
        trie.insert(pattern);
    }
    let mut cache = HashMap::new();
    towels
        .lines()
        .filter(|&towel| trie.is_possible(towel, &mut cache))
        .count()
}

pub fn part2(input: &str) -> u64 {
    let (patterns, towels) = input.trim().split_once("\n\n").unwrap();
    let mut trie = TrieNode::new();
    for pattern in patterns.split(", ") {
        trie.insert(pattern);
    }
    let mut cache: HashMap<String, u64> = HashMap::new();
    towels
        .lines()
        .map(|towel| trie.count_combinations(towel, &mut cache))
        .sum()
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
