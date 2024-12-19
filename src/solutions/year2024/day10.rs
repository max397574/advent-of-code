use std::collections::{HashMap, HashSet};

use crate::utils::grid::Grid;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let grid = Grid::from_str(input, |((_, _), c)| c as u8);
    let mut sum = 0;
    grid.iter().for_each(|(pos, c)| {
        if *c == b'0' {
            let mut reachable = HashSet::new();
            reachable.insert(pos);
            (b'1'..=b'9').for_each(|level| {
                std::mem::take(&mut reachable)
                    .iter()
                    .for_each(|&reachable_pos| {
                        grid.plus_neighbours(reachable_pos).for_each(|neighbour| {
                            if grid[neighbour] == level {
                                reachable.insert(neighbour);
                            }
                        });
                    });
            });
            sum += reachable.len();
        }
    });
    sum
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let grid = Grid::from_str(input, |((_, _), c)| c as u8);
    let mut sum = 0;
    grid.iter().for_each(|(pos, c)| {
        if *c == b'0' {
            let mut reachable = HashMap::new();
            reachable.insert(pos, 1);
            (b'1'..=b'9').for_each(|level| {
                std::mem::take(&mut reachable).iter().for_each(
                    |(&reachable_pos, &ways_to_reach)| {
                        grid.plus_neighbours(reachable_pos).for_each(|neighbour| {
                            if grid[neighbour] == level {
                                *reachable.entry(neighbour).or_insert(0) += ways_to_reach;
                            }
                        });
                    },
                );
            });
            reachable.iter().for_each(|(_, score)| {
                sum += score;
            });
        }
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("36"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("81"))
    }
}
