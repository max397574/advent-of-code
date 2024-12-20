use crate::utils::grid::Grid;

pub fn part1(input: &str) -> u32 {
    let grid = Grid::from_str(input, |(_, c)| c as u8);
    let start = grid.iter().find(|(_, &c)| c == b'S').unwrap().0;
    let end = grid.iter().find(|(_, &c)| c == b'E').unwrap().0;
    let mut path = [false; 141 * 141];
    let mut costs = [0; 141 * 141];
    let mut cost = 0;
    let mut pos = start;
    loop {
        path[pos.0 + pos.1 * 141] = true;
        costs[pos.0 + pos.1 * 141] = cost;
        cost += 1;
        if pos == end {
            break;
        }

        for neighbour in grid.plus_neighbours(pos) {
            if grid[neighbour] != b'#' && !path[neighbour.0 + 141 * neighbour.1] {
                pos = neighbour;
                break;
            }
        }
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

    for x in 0..141 {
        for y in 0..141 {
            if path[x + y * 141] {
                for (dx, dy) in CHEAT_POSITIONS {
                    let (new_x, new_y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                    if new_x < 141
                        && new_y < 141
                        && path[new_x + new_y * 141]
                        && costs[new_x + new_y * 141] + 2 < costs[x + y * 141]
                        && costs[x + y * 141] - costs[new_x + new_y * 141] - 2 >= 100
                    {
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let grid = Grid::from_str(input, |(_, c)| c as u8);
    let start = grid.iter().find(|(_, &c)| c == b'S').unwrap().0;
    let end = grid.iter().find(|(_, &c)| c == b'E').unwrap().0;
    let mut path = [false; 141 * 141];
    let mut costs = [0; 141 * 141];
    let mut cost = 0;
    let mut pos = start;
    loop {
        path[pos.0 + pos.1 * 141] = true;
        costs[pos.0 + pos.1 * 141] = cost;
        cost += 1;
        if pos == end {
            break;
        }

        for neighbour in grid.plus_neighbours(pos) {
            if grid[neighbour] != b'#' && !path[neighbour.0 + 141 * neighbour.1] {
                pos = neighbour;
                break;
            }
        }
    }

    let mut count = 0;

    for x in 0..141 {
        for y in 0..141 {
            if path[x + y * 141] {
                for dx in -20_isize..=20 {
                    let distance_left = 20 - dx.abs();
                    for dy in -distance_left..=distance_left {
                        let (new_x, new_y) =
                            ((x as isize + dx) as usize, (y as isize + dy) as usize);
                        if new_x < 141
                            && new_y < 141
                            && path[new_x + new_y * 141]
                            && costs[new_x + new_y * 141] + (dx.abs() + dy.abs())
                                < costs[x + y * 141]
                            && costs[x + y * 141]
                                - costs[new_x + new_y * 141]
                                - (dx.abs() + dy.abs())
                                >= 100
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}
