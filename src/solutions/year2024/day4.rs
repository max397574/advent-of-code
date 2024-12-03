use crate::utils::grid::Grid;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let grid = Grid::from_str(input, |(_, c)| c as u8);
    let mut count = 0;
    grid.iter().for_each(|((x, y), val)| {
        if *val == b'X' {
            let directions = [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];
            let x = x as isize;
            let y = y as isize;
            for (dx, dy) in directions {
                if grid.get_i((x + dx, y + dy)) == Some(&b'M')
                    && grid.get_i((x + dx * 2, y + dy * 2)) == Some(&b'A')
                    && grid.get_i((x + dx * 3, y + dy * 3)) == Some(&b'S')
                {
                    count += 1;
                }
            }
        }
    });
    count
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let grid = Grid::from_str(input, |(_, c)| c as u8);
    let mut count = 0;
    grid.iter().for_each(|((x, y), val)| {
        let x = x as isize;
        let y = y as isize;
        if *val == b'A'
            && ((grid.get_i((x - 1, y - 1)) == Some(&b'M')
                && grid.get_i((x + 1, y + 1)) == Some(&b'S')
                && grid.get_i((x - 1, y + 1)) == Some(&b'M')
                && grid.get_i((x + 1, y - 1)) == Some(&b'S'))
                || (grid.get_i((x - 1, y - 1)) == Some(&b'S')
                    && grid.get_i((x + 1, y + 1)) == Some(&b'M')
                    && grid.get_i((x - 1, y + 1)) == Some(&b'S')
                    && grid.get_i((x + 1, y - 1)) == Some(&b'M'))
                || (grid.get_i((x - 1, y - 1)) == Some(&b'M')
                    && grid.get_i((x + 1, y + 1)) == Some(&b'S')
                    && grid.get_i((x - 1, y + 1)) == Some(&b'S')
                    && grid.get_i((x + 1, y - 1)) == Some(&b'M'))
                || (grid.get_i((x - 1, y - 1)) == Some(&b'S')
                    && grid.get_i((x + 1, y + 1)) == Some(&b'M')
                    && grid.get_i((x - 1, y + 1)) == Some(&b'M')
                    && grid.get_i((x + 1, y - 1)) == Some(&b'S')))
        {
            count += 1;
        }
    });
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("18"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("9"))
    }
}
