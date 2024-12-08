use std::collections::HashSet;

use crate::utils::grid::Grid;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let grid = Grid::from_str(input, |((_, _), c)| c as u8);
    let mut seen_antidotes = HashSet::new();
    grid.iter().for_each(|((x, y), c)| {
        if *c != b'.' {
            grid.iter().for_each(|((x2, y2), c2)| {
                if *c == *c2 && (x, y) != (x2, y2) {
                    let (x, y, x2, y2) = (x as isize, y as isize, x2 as isize, y2 as isize);
                    let (antenna_x, antenna_y) = (x - (x2 - x), y - (y2 - y));
                    if grid.is_inbounds_i((antenna_x, antenna_y)) {
                        seen_antidotes.insert((antenna_x, antenna_y));
                    }
                    let (antenna_x, antenna_y) = (x2 + (x2 - x), y2 + (y2 - y));
                    if grid.is_inbounds_i((antenna_x, antenna_y)) {
                        seen_antidotes.insert((antenna_x, antenna_y));
                    }
                }
            });
        }
    });
    seen_antidotes.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let grid = Grid::from_str(input, |((_, _), c)| c as u8);
    let mut seen_antidotes = HashSet::new();
    grid.iter().for_each(|((x, y), c)| {
        if *c != b'.' {
            grid.iter().for_each(|((x2, y2), c2)| {
                if *c == *c2 && (x, y) != (x2, y2) {
                    let (x, y, x2, y2) = (x as isize, y as isize, x2 as isize, y2 as isize);
                    let (dx, dy) = (x2 - x, y2 - y);
                    let (mut antenna_x, mut antenna_y) = (x, y);
                    while grid.is_inbounds_i((antenna_x, antenna_y)) {
                        seen_antidotes.insert((antenna_x, antenna_y));
                        antenna_x -= dx;
                        antenna_y -= dy;
                    }
                    let (mut antenna_x, mut antenna_y) = (x2, y2);
                    while grid.is_inbounds_i((antenna_x, antenna_y)) {
                        seen_antidotes.insert((antenna_x, antenna_y));
                        antenna_x += dx;
                        antenna_y += dy;
                    }
                }
            });
        }
    });
    seen_antidotes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("14"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("34"))
    }
}
