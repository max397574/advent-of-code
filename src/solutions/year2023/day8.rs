use std::collections::HashMap;

use bstr::ByteSlice;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let mut nodes: HashMap<[u8; 3], ([u8; 3], [u8; 3])> = HashMap::new();
    for line in lines.skip(1) {
        nodes.insert(
            line[..3].try_into().unwrap(),
            (
                line[7..=9].try_into().unwrap(),
                line[12..=14].try_into().unwrap(),
            ),
        );
    }
    let mut current_node = nodes.get(b"AAA").unwrap();
    let mut steps = 0;
    loop {
        let new_node = match instructions[steps % instructions.len()] {
            b'L' => current_node.0,
            _ => current_node.1,
        };
        steps += 1;
        if new_node == *b"ZZZ" {
            break;
        }
        current_node = nodes.get(&new_node).unwrap();
    }
    steps
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let mut nodes: HashMap<[u8; 3], ([u8; 3], [u8; 3])> = HashMap::new();
    for line in lines.skip(1) {
        nodes.insert(
            line[..3].try_into().unwrap(),
            (
                line[7..=9].try_into().unwrap(),
                line[12..=14].try_into().unwrap(),
            ),
        );
    }
    let mut current_nodes = Vec::new();

    for node in nodes.keys() {
        if node[2] == b'A' {
            current_nodes.push(nodes.get(node).unwrap())
        }
    }

    let mut steps = Vec::new();
    for current_node in current_nodes.iter() {
        let mut current_node = Clone::clone(current_node);
        let mut cur_steps = 0;
        loop {
            let new_node = match instructions[cur_steps % instructions.len()] {
                b'L' => current_node.0,
                _ => current_node.1,
            };
            cur_steps += 1;
            if new_node[2] == b'Z' {
                steps.push(cur_steps);
                break;
            }
            current_node = nodes.get(&new_node).unwrap();
        }
    }
    crate::utils::get_lcm(steps)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT1).to_string(), String::from("6"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2).to_string(), String::from("6"))
    }
}
