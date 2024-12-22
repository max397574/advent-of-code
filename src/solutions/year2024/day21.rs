use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pad {
    Numpad,
    Dirpad,
}

impl Pad {
    fn pos_from_char(self, c: char) -> (i32, i32) {
        match self {
            Pad::Numpad => match c {
                '0' => (3, 1),
                '1' => (2, 0),
                '2' => (2, 1),
                '3' => (2, 2),
                '4' => (1, 0),
                '5' => (1, 1),
                '6' => (1, 2),
                '7' => (0, 0),
                '8' => (0, 1),
                '9' => (0, 2),
                'A' => (3, 2),
                _ => unreachable!(),
            },
            Pad::Dirpad => match c {
                '^' => (0, 1),
                '<' => (1, 0),
                'v' => (1, 1),
                '>' => (1, 2),
                'A' => (0, 2),
                _ => unreachable!(),
            },
        }
    }
}

fn gen_paths_rev(
    start: (i32, i32),
    end: (i32, i32),
    in_bounds: &dyn Fn((i32, i32)) -> bool,
) -> Vec<String> {
    if !in_bounds(start) {
        return vec![];
    }
    if start == end {
        return vec!["A".to_string()];
    }
    let mut ret = Vec::new();
    if end.0 < start.0 {
        for s in gen_paths_rev((start.0 - 1, start.1), end, in_bounds) {
            ret.push(s + "^");
        }
    }
    if end.0 > start.0 {
        for s in gen_paths_rev((start.0 + 1, start.1), end, in_bounds) {
            ret.push(s + "v");
        }
    }
    if end.1 < start.1 {
        for s in gen_paths_rev((start.0, start.1 - 1), end, in_bounds) {
            ret.push(s + "<");
        }
    }
    if end.1 > start.1 {
        for s in gen_paths_rev((start.0, start.1 + 1), end, in_bounds) {
            ret.push(s + ">");
        }
    }
    ret
}

fn gen_paths(
    start: (i32, i32),
    end: (i32, i32),
    in_bounds: &dyn Fn((i32, i32)) -> bool,
) -> Vec<String> {
    let mut ret = gen_paths_rev(start, end, in_bounds);
    for s in &mut ret {
        *s = s.chars().rev().collect::<String>();
    }
    ret
}

fn in_numpad(p: (i32, i32)) -> bool {
    if p == (3, 0) {
        return false;
    }
    p.0 >= 0 && p.0 < 4 && p.1 >= 0 && p.1 < 3
}

fn in_dirpad(p: (i32, i32)) -> bool {
    if p == (0, 0) {
        return false;
    }
    p.0 >= 0 && p.0 < 2 && p.1 >= 0 && p.1 < 3
}

fn rec(
    s: char,
    e: char,
    l: i32,
    cache: &mut HashMap<((char, char), i32), i64>,
    layers: i32,
) -> i64 {
    if l == layers - 1 {
        return 1;
    }
    let key = ((s, e), l);
    if let Some(&value) = cache.get(&key) {
        return value;
    }
    let mut best = -1;
    let pad = if l == 0 { Pad::Numpad } else { Pad::Dirpad };
    let paths = gen_paths(
        pad.pos_from_char(s),
        pad.pos_from_char(e),
        if l == 0 { &in_numpad } else { &in_dirpad },
    );
    for path in paths {
        let mut len = 0;
        let mut cur = 'A';
        for goal in path.chars() {
            len += rec(cur, goal, l + 1, cache, layers);
            cur = goal;
        }
        if best == -1 || len < best {
            best = len;
        }
    }
    cache.insert(key, best);
    best
}

pub fn part1(input: &str) -> i64 {
    let mut ret = 0;
    for line in input.lines() {
        let mut len = 0;
        let mut cur = 'A';
        let mut cache = HashMap::new();
        for goal in line.chars() {
            len += rec(cur, goal, 0, &mut cache, 4);
            cur = goal;
        }
        ret += line[..3].parse::<i64>().unwrap() * len;
    }
    ret
}

pub fn part2(input: &str) -> i64 {
    let mut ret = 0;
    for line in input.lines() {
        let mut len = 0;
        let mut cur = 'A';
        let mut memo = HashMap::new();
        for goal in line.chars() {
            len += rec(cur, goal, 0, &mut memo, 27);
            cur = goal;
        }
        ret += line[..3].parse::<i64>().unwrap() * len;
    }
    ret
}

// #[cfg(test)]
mod tests {
    use super::*;
    const _INPUT: &str = "";

    // #[test]
    fn _part1() {
        assert_eq!(part1(_INPUT).to_string(), String::from("0"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(_INPUT).to_string(), String::from("0"))
    }
}
