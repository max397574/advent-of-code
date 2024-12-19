use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::grid::Grid;
use crate::utils::Direction;

struct Path {
    pub path: Vec<(usize, usize)>,
    pub cost: u32,
}

fn walk(input: &str) -> Vec<Path> {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = Grid::from_str(input, |(_, c)| c as u8);
    grid.iter().for_each(|((x, y), &c)| {
        if c == b'S' {
            start = (x, y);
        } else if c == b'E' {
            end = (x, y);
        }
    });
    let mut min_seen: HashMap<((usize, usize), Direction), u32> = HashMap::new();
    let mut queue = VecDeque::from([(start, 0, Direction::Right, vec![start])]);
    let mut min_cost = u32::MAX;
    let mut paths = Vec::new();
    while let Some(entry) = queue.pop_front() {
        if entry.1 <= *min_seen.entry((entry.0, entry.2)).or_insert(u32::MAX) {
            min_seen.insert((entry.0, entry.2), entry.1);
        }

        if entry.0 == end {
            paths.push(Path {
                path: entry.3.clone(),
                cost: entry.1,
            });
            min_cost = min_cost.min(entry.1);
            continue;
        }

        if entry.1 > min_cost {
            continue;
        }

        let (x, y) = entry.0;

        match entry.2 {
            Direction::Right => {
                if x < grid.width - 1
                    && grid[(x + 1, y)] != b'#'
                    && entry.1
                        < *min_seen
                            .entry(((x + 1, y), Direction::Right))
                            .or_insert(u32::MAX)
                {
                    let mut new_path = entry.3.clone();
                    new_path.push((x + 1, y));
                    queue.push_front(((x + 1, y), entry.1 + 1, Direction::Right, new_path.clone()));
                }
            }
            Direction::Left => {
                if x > 0
                    && grid[(x - 1, y)] != b'#'
                    && entry.1
                        < *min_seen
                            .entry(((x - 1, y), Direction::Left))
                            .or_insert(u32::MAX)
                {
                    let mut new_path = entry.3.clone();
                    new_path.push((x - 1, y));
                    queue.push_front(((x - 1, y), entry.1 + 1, Direction::Left, new_path.clone()));
                }
            }
            Direction::Down => {
                if y < grid.height() - 1
                    && grid[(x, y + 1)] != b'#'
                    && entry.1
                        < *min_seen
                            .entry(((x, y + 1), Direction::Down))
                            .or_insert(u32::MAX)
                {
                    let mut new_path = entry.3.clone();
                    new_path.push((x, y + 1));
                    queue.push_front(((x, y + 1), entry.1 + 1, Direction::Down, new_path.clone()));
                }
            }
            Direction::Up => {
                if y > 0
                    && grid[(x, y - 1)] != b'#'
                    && entry.1
                        < *min_seen
                            .entry(((x, y - 1), Direction::Up))
                            .or_insert(u32::MAX)
                {
                    let mut new_path = entry.3.clone();
                    new_path.push((x, y - 1));
                    queue.push_front(((x, y - 1), entry.1 + 1, Direction::Up, new_path.clone()));
                }
            }
        }

        if entry.1 + 1000
            <= *min_seen
                .entry(((x, y), entry.2.turn_left()))
                .or_insert(u32::MAX)
        {
            queue.push_back(((x, y), entry.1 + 1000, entry.2.turn_left(), entry.3.clone()));
        }

        if entry.1 + 1000
            <= *min_seen
                .entry(((x, y), entry.2.turn_right()))
                .or_insert(u32::MAX)
        {
            queue.push_back((
                (x, y),
                entry.1 + 1000,
                entry.2.turn_right(),
                entry.3.clone(),
            ));
        }

        if entry.1 + 2000
            <= *min_seen
                .entry(((x, y), entry.2.turn_right().turn_right()))
                .or_insert(u32::MAX)
        {
            queue.push_back((
                (x, y),
                entry.1 + 2000,
                entry.2.turn_right().turn_right(),
                entry.3.clone(),
            ));
        }
    }
    paths
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let paths = walk(input);
    paths.iter().map(|path| path.cost).min().unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let paths = walk(input);
    let mut seen = HashSet::new();
    //let mut grid = Grid::from_str(input, |(_, c)| c);
    let min = paths.iter().map(|path| path.cost).min().unwrap();
    paths.iter().for_each(|path| {
        if path.cost == min {
            path.path.iter().for_each(|pos| {
                seen.insert(pos);
                //grid.set_at((pos.0, pos.1), 'O');
            });
        }
    });
    //println!("{grid}");
    seen.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("7036"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("45"))
    }
}
