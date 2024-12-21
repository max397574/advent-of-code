use crate::utils::grid::Grid;

pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    unsafe {
        let input = input.as_bytes();
        let mut start = 0;
        let mut end = 0;
        for (i, &byte) in input.iter().enumerate() {
            if byte == b'S' {
                start = i;
            }
            if byte == b'E' {
                end = i;
            }
        }
        let mut path = [false; 141 * 142];
        let mut costs = [0; 141 * 142];
        let mut cost = 0;
        let mut pos = start;
        loop {
            *path.get_unchecked_mut(pos) = true;
            *costs.get_unchecked_mut(pos) = cost;
            cost += 1;
            if pos == end {
                break;
            }

            const NEIGHBOUR_OFFSETS: [isize; 4] = [-142, 142, 1, -1];
            for offset in NEIGHBOUR_OFFSETS {
                let new_pos = pos.wrapping_add_signed(offset);
                if new_pos < 141 * 142 - 1
                    && *input.get_unchecked(new_pos) != b'\n'
                    && *input.get_unchecked(new_pos) != b'#'
                    && !path.get_unchecked(new_pos)
                {
                    pos = new_pos;
                    break;
                }
            }
        }

        // iterate through all positions and figure out if a cheat could have saved time
        const CHEAT_POSITIONS: [isize; 8] = [-284, 284, 2, -2, 143, -143, 141, -141];

        for x in 0..141 {
            for y in 0..141 {
                let pos = x + y * 142;
                if path[pos] {
                    for offset in CHEAT_POSITIONS {
                        let new_pos = pos.wrapping_add_signed(offset);
                        if *path.get_unchecked(new_pos)
                            && costs.get_unchecked(new_pos) + 2 < *costs.get_unchecked(pos)
                            && costs.get_unchecked(pos) - costs.get_unchecked(new_pos) - 2 >= 100
                        {
                            sum += 1;
                        }
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
