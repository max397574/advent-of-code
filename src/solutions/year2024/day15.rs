use indexmap::IndexSet;

use crate::utils::grid::Grid;

pub type FxIndexSet<T> = IndexSet<T, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

pub fn part1(input: &str) -> usize {
    let (grid, ops) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::from_str(grid, |(_, c)| c as u8);
    let ((x, y), _) = grid.iter().find(|(_, &c)| c == b'@').unwrap();
    grid.set_at((x, y), b'.');
    let (mut grid, (x, y), ops) = (grid, (x, y), ops.trim().as_bytes());

    let (mut x, mut y) = (x as isize, y as isize);
    for &op in ops {
        let (dx, dy) = match op {
            b'^' => (0, -1),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'\n' => continue,
            _ => unreachable!(),
        };
        let (mut fx, mut fy) = (x + dx, y + dy);
        loop {
            match grid[(fx, fy)] {
                b'O' => (fx, fy) = (fx + dx, fy + dy),
                b'.' => {
                    (x, y) = (x + dx, y + dy);
                    grid.set_at_i((fx, fy), b'O');
                    grid.set_at_i((x, y), b'.');
                    break;
                }
                b'#' => {
                    break;
                }
                _ => unreachable!(),
            }
        }
    }

    grid.iter()
        .filter(|(_, &c)| c == b'O')
        .map(|((x, y), _)| 100 * y + x)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (grid, ops) = input.split_once("\n\n").unwrap();
    let mut double_wide_grid = String::new();
    for line in grid.lines() {
        let mut double = String::new();
        for char in line.chars() {
            match char {
                'O' => {
                    double.push('[');
                    double.push(']');
                }
                '@' => {
                    double.push('@');
                    double.push('.');
                }
                _ => {
                    double.push(char);
                    double.push(char);
                }
            }
        }
        double_wide_grid.push_str(&double);
        double_wide_grid.push('\n');
    }
    let grid = double_wide_grid.trim();
    let mut grid = Grid::from_str(grid, |(_, c)| c as u8);
    let ((x, y), _) = grid.iter().find(|(_, &c)| c == b'@').unwrap();
    grid.set_at((x, y), b'.');
    let (mut grid, (x, y), ops) = (grid, (x, y), ops.trim().as_bytes());

    let (mut x, mut y) = (x as isize, y as isize);
    for &op in ops {
        let (dx, dy) = match op {
            b'^' => (0, -1),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'\n' => continue,
            _ => unreachable!(),
        };

        /// x,y: pos we want to move to
        /// returns whether we can move there or not
        fn can_move_to(grid: &mut Grid<u8>, x: isize, y: isize, dx: isize, dy: isize) -> bool {
            match grid.get_i((x, y)).unwrap() {
                b'#' => false,
                b'.' => true,
                b'[' => {
                    if dx == 0 {
                        if can_move_to(grid, x, y + dy, dx, dy)
                            && can_move_to(grid, x + 1, y + dy, dx, dy)
                        {
                            grid.set_at_i((x, y), b'.');
                            grid.set_at_i((x + 1, y), b'.');
                            grid.set_at_i((x, y + dy), b'[');
                            grid.set_at_i((x + 1, y + dy), b']');
                            true
                        } else {
                            false
                        }
                    } else if can_move_to(grid, x + 2, y, dx, dy) {
                        grid.set_at_i((x, y), b'.');
                        grid.set_at_i((x + 1, y), b'[');
                        grid.set_at_i((x + 2, y), b']');
                        true
                    } else {
                        false
                    }
                }
                b']' => {
                    if dx == 0 {
                        if can_move_to(grid, x, y + dy, dx, dy)
                            && can_move_to(grid, x - 1, y + dy, dx, dy)
                        {
                            grid.set_at_i((x, y), b'.');
                            grid.set_at_i((x - 1, y), b'.');
                            grid.set_at_i((x, y + dy), b']');
                            grid.set_at_i((x - 1, y + dy), b'[');
                            true
                        } else {
                            false
                        }
                    } else if can_move_to(grid, x - 2, y, dx, dy) {
                        grid.set_at_i((x - 2, y), b'[');
                        grid.set_at_i((x - 1, y), b']');
                        grid.set_at_i((x, y), b'.');
                        true
                    } else {
                        false
                    }
                }
                _ => unreachable!(),
            }
        }

        let mut new_grid = grid.clone();
        if can_move_to(&mut new_grid, x + dx, y + dy, dx, dy) {
            for ((x, y), &c) in new_grid.iter() {
                if c == b'[' {
                    assert_eq!(new_grid[(x + 1, y)], b']');
                }
            }
            grid = new_grid;
            (x, y) = (x + dx, y + dy);
        }
    }

    grid.iter()
        .filter(|(_, &c)| c == b'[')
        .map(|((x, y), _)| 100 * y + x)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("10092"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("9021"))
    }
}
