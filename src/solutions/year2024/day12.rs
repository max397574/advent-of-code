use std::collections::HashSet;

use crate::utils::grid::Grid;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let grid = Grid::from_str(input, |((_, _), c)| c as u8);
    let mut seen = HashSet::new();
    let mut score = 0;
    grid.iter().for_each(|((x, y), &c)| {
        if seen.insert((x, y)) {
            let mut total_perimeter = 0;
            let mut area = 1;
            let mut queue = Vec::from([(x, y)]);
            while let Some((x, y)) = queue.pop() {
                let mut perimeter = 4;
                for neighbour in grid.plus_neighbours((x, y)) {
                    if grid[neighbour] == c {
                        perimeter -= 1;
                        if seen.insert(neighbour) {
                            area += 1;
                            queue.push(neighbour);
                        }
                    }
                }
                total_perimeter += perimeter;
            }
            score += area * total_perimeter;
        }
    });
    score
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let grid = Grid::from_str(input, |((_, _), c)| c as u8);
    let mut seen = HashSet::new();
    let mut score = 0;
    grid.iter().for_each(|((x, y), &c)| {
        if seen.insert((x, y)) {
            let mut area = 1;
            let mut queue = Vec::from([(x, y)]);
            let mut corners = 0;
            while let Some((x, y)) = queue.pop() {
                for neighbour in grid.plus_neighbours((x, y)) {
                    if grid[neighbour] == c && seen.insert(neighbour) {
                        area += 1;
                        queue.push(neighbour);
                    }
                }
                let (x, y) = (x as isize, y as isize);
                for (dx, dy) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
                    if let Some(&c2) = grid.get_at_i((x + dx, y))
                        && let Some(&c3) = grid.get_at_i((x, y + dy))
                        && c2 == c3
                        && let Some(&c4) = grid.get_at_i((x + dx, y + dy))
                        && !(c3 == c4 && c == c3)
                    {
                        corners += 1;
                    } else if let Some(&c2) = grid.get_at_i((x + dx, y))
                        && let Some(&c3) = grid.get_at_i((x, y + dy))
                        && c2 != c
                        && c3 != c
                    {
                        corners += 1;
                    } else if grid.get_at_i((x + dx, y + dy)).is_none()
                        && grid.get_at_i((x + dx, y)).is_none()
                        && grid.get_at_i((x, y + dy)).is_none()
                    {
                        corners += 1;
                    } else if grid.get_at_i((x + dx, y)).is_none()
                        && let Some(&c2) = grid.get_at_i((x, y + dy))
                        && c2 != c
                    {
                        corners += 1;
                    } else if grid.get_at_i((x, y + dy)).is_none()
                        && let Some(&c2) = grid.get_at_i((x + dx, y))
                        && c2 != c
                    {
                        corners += 1;
                    }
                }
            }
            score += area * corners;
        }
    });
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const INPUT2: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("1930"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("368"))
    }

    #[test]
    fn part_2_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("1206"))
    }
}
