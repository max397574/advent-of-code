use crate::aoc_helpers::grid::Grid;

fn solve(input: &str, mut expansion_factor: usize) -> usize {
    let grid = Grid::from_str(input, |((_, _), c)| c == '#');
    let empty_cols = (0..grid.width)
        .filter(|&x| (0..grid.height()).all(|y| !grid[(x, y)]))
        .collect::<Vec<_>>();
    let empty_rows = (0..grid.height())
        .filter(|&y| (0..grid.width).all(|x| !grid[(x, y)]))
        .collect::<Vec<_>>();

    let galaxies = grid
        .iter()
        .filter_map(|((x, y), &is_galaxy)| {
            if !is_galaxy {
                return None;
            }
            if expansion_factor <= 1 {
                expansion_factor += 1;
            }
            let x = x + empty_cols.iter().filter(|&&col| col < x).count() * (expansion_factor - 1);
            let y = y + empty_rows.iter().filter(|&&row| row < y).count() * (expansion_factor - 1);
            Some((x, y))
        })
        .collect::<Vec<_>>();

    galaxies
        .iter()
        .enumerate()
        .map(|(idx, galaxy)| {
            galaxies[idx + 1..]
                .iter()
                .map(|galaxy2| galaxy.0.abs_diff(galaxy2.0) + galaxy.1.abs_diff(galaxy2.1))
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    solve(input, 1)
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    solve(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1() {
        assert_eq!(solve(INPUT, 1).to_string(), String::from("374"))
    }

    #[test]
    fn part2_factor10() {
        assert_eq!(solve(INPUT, 10).to_string(), String::from("1030"))
    }

    #[test]
    fn part2_factor100() {
        assert_eq!(solve(INPUT, 100).to_string(), String::from("8410"))
    }
}
