use crate::utils::{grid::Grid, Direction};

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let mut player_pos: (isize, isize) = (0, 0);
    let mut direction = Direction::Up;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let coords = (x as isize, y as isize);
            match c {
                '<' => {
                    player_pos = coords;
                    direction = Direction::Left;
                }
                '>' => {
                    player_pos = coords;
                    direction = Direction::Right;
                }
                '^' => {
                    player_pos = coords;
                    direction = Direction::Up;
                }
                'v' => {
                    player_pos = coords;
                    direction = Direction::Down;
                }
                _ => (),
            };
        })
    });

    let mut grid = Grid::from_str(input, |(_, val)| val);

    grid.set_at_i(player_pos, 'x');

    let mut count = 1;
    while let Some(val) = grid.get_at_i((
        player_pos.0 + direction.get_offset().0,
        player_pos.1 + direction.get_offset().1,
    )) {
        match val {
            '#' => {
                direction = direction.turn_right();
            }
            _ => {
                player_pos = (
                    player_pos.0 + direction.get_offset().0,
                    player_pos.1 + direction.get_offset().1,
                );
                if *val != 'x' {
                    count += 1;
                }
                grid.set_at_i(player_pos, 'x');
            }
        };
    }
    count
}

pub fn has_loop(mut grid: Grid<char>, player_pos: (isize, isize), direction: Direction) -> bool {
    let mut player_pos = player_pos;
    let mut direction = direction;
    while let Some(val) = grid.get_at_i((
        player_pos.0 + direction.get_offset().0,
        player_pos.1 + direction.get_offset().1,
    )) {
        match val {
            '#' => {
                direction = direction.turn_right();
            }
            _ => {
                player_pos = (
                    player_pos.0 + direction.get_offset().0,
                    player_pos.1 + direction.get_offset().1,
                );
                if *val != '.'
                    && *val
                        == match direction {
                            Direction::Up => '^',
                            Direction::Left => '<',
                            Direction::Right => '>',
                            Direction::Down => 'v',
                        }
                {
                    return true;
                }
                if *val == '.' {
                    grid.set_at_i(
                        player_pos,
                        match direction {
                            Direction::Up => '^',
                            Direction::Left => '<',
                            Direction::Right => '>',
                            Direction::Down => 'v',
                        },
                    );
                }
            }
        };
    }
    false
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let mut player_pos: (isize, isize) = (0, 0);
    let mut direction = Direction::Up;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let coords = (x as isize, y as isize);
            match c {
                '<' => {
                    player_pos = coords;
                    direction = Direction::Left;
                }
                '>' => {
                    player_pos = coords;
                    direction = Direction::Right;
                }
                '^' => {
                    player_pos = coords;
                    direction = Direction::Up;
                }
                'v' => {
                    player_pos = coords;
                    direction = Direction::Down;
                }
                _ => (),
            };
        })
    });
    let start_pos = player_pos;
    let start_direction = direction;

    let mut grid = Grid::from_str(input, |(_, val)| val);

    grid.set_at_i(
        player_pos,
        match direction {
            Direction::Up => '^',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Down => 'v',
        },
    );

    let og_grid = Grid {
        cells: grid.cells.clone(),
        width: grid.width,
    };

    let mut visited = vec![player_pos];

    while let Some(val) = grid.get_at_i((
        player_pos.0 + direction.get_offset().0,
        player_pos.1 + direction.get_offset().1,
    )) {
        match val {
            '#' => {
                direction = direction.turn_right();
            }
            _ => {
                player_pos = (
                    player_pos.0 + direction.get_offset().0,
                    player_pos.1 + direction.get_offset().1,
                );
                if *val != 'x' {
                    visited.push(player_pos);
                }
                grid.set_at_i(player_pos, 'x');
            }
        };
    }

    let mut count = 0;
    visited.iter().for_each(|pos| {
        let mut tmp_grid = Grid {
            cells: og_grid.cells.clone(),
            width: og_grid.width,
        };
        tmp_grid.set_at_i(*pos, '#');
        if has_loop(
            Grid {
                cells: tmp_grid.cells.clone(),
                width: grid.width,
            },
            start_pos,
            start_direction,
        ) {
            count += 1;
        }
    });
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("41"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("6"))
    }
}
