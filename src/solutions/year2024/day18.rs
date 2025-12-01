use std::collections::{HashSet, VecDeque};

use bstr::ByteSlice;

use crate::utils::parsing::ByteParsing;

const SIZE: u8 = 70;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    get_path(input, 1024).unwrap()
}

fn get_path(input: &str, iterations: usize) -> Option<i32> {
    let mut lines = input.as_bytes().lines();
    let mut i = 0;
    let mut blocked = HashSet::new();
    while i < iterations {
        let (x, y) = lines.next().unwrap().split_once(|&c| c == b',').unwrap();
        blocked.insert((x.as_num::<u8>(), y.as_num::<u8>()));
        i += 1;
    }

    let mut queue = VecDeque::from([(0, (0, 0))]);
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if pos.1 == (SIZE, SIZE) {
            return Some(pos.0);
        }

        // if is blocked or already visited
        if !blocked.insert(pos.1) {
            continue;
        }

        if (pos.1).0 > 0 {
            queue.push_back((pos.0 + 1, ((pos.1).0 - 1, (pos.1).1)))
        }
        if (pos.1).1 > 0 {
            queue.push_back((pos.0 + 1, ((pos.1).0, (pos.1).1 - 1)))
        }
        if (pos.1).0 < SIZE {
            queue.push_back((pos.0 + 1, ((pos.1).0 + 1, (pos.1).1)))
        }
        if (pos.1).1 < SIZE {
            queue.push_back((pos.0 + 1, ((pos.1).0, (pos.1).1 + 1)))
        }
    }
    None
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let mut lines = input.as_bytes().lines();
    let mut min = 0;
    let mut max = lines.clone().count();
    let mut i = min + (max - min) / 2;
    loop {
        if get_path(input, i).is_some() && get_path(input, i + 1).is_none() {
            break;
        }
        if get_path(input, i).is_some() {
            min = i + 1;
        } else {
            max = i;
        }
        i = min + (max - min) / 2;
    }
    lines.nth(i).unwrap().to_str().unwrap().to_owned()
}
