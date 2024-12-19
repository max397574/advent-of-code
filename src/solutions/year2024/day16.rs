use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::grid::Grid;
use crate::utils::Direction;

struct Path {
    pub path: Vec<(usize, usize)>,
    pub cost: u32,
}

fn find_min_paths(input: &str) -> Vec<Path> {
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

        let is_not_oob = |direction| match direction {
            Direction::Right => x < grid.width - 1,
            Direction::Left => x > 0,
            Direction::Down => y < grid.height() - 1,
            Direction::Up => y > 0,
        };

        let (dx, dy) = entry.2.get_offset();

        let new_x = (x as isize + dx) as usize;
        let new_y = (y as isize + dy) as usize;

        if is_not_oob(entry.2)
            && grid[(new_x, new_y)] != b'#'
            && entry.1
                < *min_seen
                    .entry(((new_x, new_y), entry.2))
                    .or_insert(u32::MAX)
        {
            let mut new_path = entry.3.clone();
            new_path.push((new_x, new_y));
            queue.push_front(((new_x, new_y), entry.1 + 1, entry.2, new_path.clone()));
        }

        let mut try_turn = |i| {
            let (new_direction, cost) = match i {
                1 => (entry.2.turn_left(), 1000),
                2 => (entry.2.turn_left().turn_left(), 2000),
                3 => (entry.2.turn_right(), 1000),
                _ => unreachable!(),
            };
            if entry.1 + cost <= *min_seen.entry(((x, y), new_direction)).or_insert(u32::MAX) {
                queue.push_back(((x, y), entry.1 + cost, new_direction, entry.3.clone()));
            }
        };

        try_turn(1);
        try_turn(2);
        try_turn(3);
    }
    paths
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let paths = find_min_paths(input);
    paths.iter().map(|path| path.cost).min().unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let paths = find_min_paths(input);
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
