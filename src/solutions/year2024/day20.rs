use std::collections::{HashMap, VecDeque};

use crate::utils::grid::Grid;

pub fn part1(input: &str) -> u32 {
    let grid = Grid::from_str(input, |(_, c)| c as u8);
    let start = grid.iter().find(|(_, &c)| c == b'S').unwrap().0;
    let end = grid.iter().find(|(_, &c)| c == b'E').unwrap().0;
    let mut queue = VecDeque::from([(start, 0)]);
    let mut min_costs = HashMap::new();
    while let Some((pos, cost)) = queue.pop_front() {
        // because we use BFS we know we already only have minimal distances
        if min_costs.contains_key(&pos) {
            continue;
        }

        min_costs.insert(pos, cost);

        if pos == end {
            break;
        }

        grid.plus_neighbours(pos).for_each(|neighbour| {
            if grid[neighbour] != b'#' {
                queue.push_back((neighbour, cost + 1));
            }
        })
    }

    // iterate through all positions and figure out if a cheat could have saved time
    const CHEAT_POSITIONS: [(isize, isize); 8] = [
        (2, 0),
        (0, 2),
        (-2, 0),
        (0, -2),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut sum = 0;

    grid.iter().for_each(|((x, y), _)| {
        if let Some(&cost) = min_costs.get(&(x, y)) {
            let (x, y) = (x as isize, y as isize);
            for (dx, dy) in CHEAT_POSITIONS {
                let (new_x, new_y) = (x + dx, y + dy);
                if new_x >= 0
                    && new_y >= 0
                    && new_x < grid.width as isize
                    && new_y < grid.height() as isize
                {
                    if let Some(cheated_cost) = min_costs.get(&(new_x as usize, new_y as usize)) {
                        if cheated_cost + 2 < cost && cost - cheated_cost - 2 >= 100 {
                            sum += 1;
                        }
                    }
                }
            }
        }
    });
    sum
}

pub fn part2(input: &str) -> u32 {
    let grid = Grid::from_str(input, |(_, c)| c as u8);
    let start = grid.iter().find(|(_, &c)| c == b'S').unwrap().0;
    let end = grid.iter().find(|(_, &c)| c == b'E').unwrap().0;
    let mut queue = VecDeque::from([(start, 0)]);
    let mut min_costs = HashMap::new();
    while let Some((pos, cost)) = queue.pop_front() {
        // because we use BFS we know we already only have minimal distances
        if min_costs.contains_key(&pos) {
            continue;
        }

        min_costs.insert(pos, cost);

        if pos == end {
            break;
        }

        grid.plus_neighbours(pos).for_each(|neighbour| {
            if grid[neighbour] != b'#' {
                queue.push_back((neighbour, cost + 1));
            }
        })
    }

    let mut count = 0;

    grid.iter().for_each(|((x, y), _)| {
        if let Some(&cost) = min_costs.get(&(x, y)) {
            let (x, y) = (x as isize, y as isize);
            // search around pos to a depth of 20
            for dx in -20_isize..=20 {
                let distance_left = 20 - dx.abs();
                for dy in -distance_left..=distance_left {
                    let (new_x, new_y) = (x + dx, y + dy);
                    if new_x >= 0
                        && new_y >= 0
                        && new_x < grid.width as isize
                        && new_y < grid.height() as isize
                    {
                        if let Some(cheated_cost) = min_costs.get(&(new_x as usize, new_y as usize))
                        {
                            if cheated_cost + (dx.abs() + dy.abs()) < cost {
                                let saved = cost - cheated_cost - (dx.abs() + dy.abs());
                                if saved >= 100 {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("0"))
    }

    //#[test]
    fn _part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("0"))
    }
}
