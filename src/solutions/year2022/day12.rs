use std::collections::{HashSet, VecDeque};

fn get_distance(parents: &[Option<usize>], index: usize, depth: usize) -> usize {
    match parents[index] {
        Some(parent) => get_distance(parents, parent, depth + 1),
        None => depth,
    }
}

pub fn part_1(_input: &str) -> impl std::fmt::Display {
    let start = steps.iter().position(|step| step.height == 50).unwrap();
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    let mut parents = vec![None; steps.len()];
    explored.insert(start);
    queue.push_back(start);
    while let Some(index) = queue.pop_front() {
        let step = &steps[index];
        if step.height == 100 {
            return get_distance(&parents, index, 0);
        }
        let height = match step.height {
            100 => 26,
            50 => 1,
            _ => step.height,
        };
        for edge in step.edges.iter() {
            let edge_height = match steps[*edge].height {
                100 => 26,
                50 => 1,
                x => x,
            };
            if edge_height <= height + 1 && explored.insert(*edge) {
                parents[*edge] = Some(index);
                queue.push_back(*edge);
            }
        }
    }
    0
}

// Can be solved to search the shortest path to value 1 from `E`
pub fn part_2(_input: &str) -> impl std::fmt::Display {
    let start = steps.iter().position(|step| step.height == 100).unwrap();
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    let mut parents = vec![None; steps.len()];
    explored.insert(start);
    queue.push_back(start);
    while let Some(index) = queue.pop_front() {
        let step = &steps[index];
        let height = match step.height {
            100 => 26,
            50 => 1,
            _ => step.height,
        };
        if height == 1 {
            return get_distance(&parents, index, 0);
        }
        for edge in step.edges.iter() {
            let edge_height = match steps[*edge].height {
                100 => 26,
                50 => 1,
                x => x,
            };
            if edge_height >= height - 1 && explored.insert(*edge) {
                parents[*edge] = Some(index);
                queue.push_back(*edge);
            }
        }
    }
    0
}

// #[cfg(test)]
mod tests {
    use super::*;
    const _INPUT1: &str = "";
    const _INPUT2: &str = "";

    // #[test]
    fn _part1() {
        assert_eq!(part_1(_INPUT1).to_string(), String::from("0"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part_2(_INPUT2).to_string(), String::from("0"))
    }
}
