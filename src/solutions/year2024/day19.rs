use std::collections::HashMap;

const CHAR_MAP: [u8; 256] = {
    let mut map = [5u8; 256];
    map[b'r' as usize] = 0;
    map[b'g' as usize] = 1;
    map[b'b' as usize] = 2;
    map[b'u' as usize] = 3;
    map[b'w' as usize] = 4;
    map
};

struct TrieNode {
    children: [Option<Box<TrieNode>>; 5],
    is_leaf: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            is_leaf: false,
        }
    }

    fn insert(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.is_leaf = true;
            return;
        }
        let first = word[0] as usize;
        if self.children[first].is_none() {
            self.children[first] = Some(Box::new(TrieNode::new()));
        }
        self.children[first].as_mut().unwrap().insert(&word[1..]);
    }

    fn is_possible(&self, towel: &[u8], cache: &mut HashMap<Vec<u8>, bool>) -> bool {
        if towel.is_empty() {
            return true;
        }

        if let Some(&result) = cache.get(towel) {
            return result;
        }

        let mut current = self;
        for (i, &ch) in towel.iter().enumerate() {
            if let Some(next) = &current.children[ch as usize] {
                current = next;
                if current.is_leaf && self.is_possible(&towel[i + 1..], cache) {
                    cache.insert(towel.to_vec(), true);
                    return true;
                }
            } else {
                cache.insert(towel.to_vec(), false);
                return false;
            }
        }
        cache.insert(towel.to_vec(), false);
        false
    }

    fn count_combinations(&self, towel: &[u8], cache: &mut HashMap<Vec<u8>, u64>) -> u64 {
        if towel.is_empty() {
            return 1;
        }

        if let Some(&count) = cache.get(towel) {
            return count;
        }

        let mut count = 0;
        let mut current = self;
        for (i, &ch) in towel.iter().enumerate() {
            if let Some(next) = &current.children[ch as usize] {
                current = next;
                if current.is_leaf {
                    count += self.count_combinations(&towel[i + 1..], cache);
                }
            } else {
                break;
            }
        }
        cache.insert(towel.to_vec(), count);
        count
    }
}

pub fn part1(input: &str) -> usize {
    let (patterns, towels) = input.trim().split_once("\n\n").unwrap();
    let mut trie = TrieNode::new();
    for pattern in patterns.split(", ") {
        trie.insert(
            &pattern
                .bytes()
                .map(|b| CHAR_MAP[b as usize])
                .collect::<Vec<u8>>(),
        );
    }
    let mut cache = HashMap::new();
    towels
        .lines()
        .filter(|&towel| {
            let towel_bytes = towel
                .bytes()
                .map(|b| CHAR_MAP[b as usize])
                .collect::<Vec<u8>>();
            trie.is_possible(&towel_bytes, &mut cache)
        })
        .count()
}

pub fn part2(input: &str) -> u64 {
    let (patterns, towels) = input.trim().split_once("\n\n").unwrap();
    let mut trie = TrieNode::new();
    for pattern in patterns.split(", ") {
        trie.insert(
            &pattern
                .bytes()
                .map(|b| CHAR_MAP[b as usize])
                .collect::<Vec<u8>>(),
        );
    }
    let mut cache = HashMap::new();
    towels
        .lines()
        .map(|towel| {
            let towel_bytes = towel
                .bytes()
                .map(|b| CHAR_MAP[b as usize])
                .collect::<Vec<u8>>();
            trie.count_combinations(&towel_bytes, &mut cache)
        })
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
