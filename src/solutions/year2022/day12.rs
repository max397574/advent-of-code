use std::collections::{HashSet, VecDeque};

use crate::utils::grid;

#[derive(Debug)]
struct Step {
    edges: Vec<usize>,
    height: u8,
}

fn get_steps(input: &str) -> Vec<Step> {
    let grid = grid(input, |x| {
        if x == 'E' {
            return 100;
        } else if x == 'S' {
            return 50;
        }
        x as u8 - b'a' + 1
    });

    let width = grid[0].len();
    let height = grid.len();
    let mut steps = Vec::with_capacity(width * height);
    for row in 0..height {
        for col in 0..width {
            let value = grid[row][col];
            let mut adjacent_indices = vec![(row + 1, col), (row, col + 1)];
            if row > 0 {
                adjacent_indices.push((row - 1, col));
            }
            if col > 0 {
                adjacent_indices.push((row, col - 1));
            }
            let adjacent_indices =
                adjacent_indices
                    .into_iter()
                    .filter(|(row, col)| match grid.get(*row) {
                        Some(row) => row.get(*col).is_some(),
                        None => false,
                    });
            steps.push(Step {
                edges: adjacent_indices
                    .map(|(row, col)| row * width + col)
                    .collect(),
                height: value,
            });
        }
    }
    steps
}

fn get_distance(parents: &[Option<usize>], index: usize, depth: usize) -> usize {
    match parents[index] {
        Some(parent) => get_distance(parents, parent, depth + 1),
        None => depth,
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let steps = get_steps(input);
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
pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let steps = get_steps(input);
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

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("31"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("29"))
    }
}
