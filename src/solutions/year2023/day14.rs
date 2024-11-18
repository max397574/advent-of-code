use crate::utils::grid::Grid;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash, PartialEq, Eq)]
enum Type {
    Round,
    Square,
    Empty,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Round => "O",
                Type::Square => "#",
                Type::Empty => ".",
            }
        )
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut grid = Grid::from_str(input, |((_, _), c)| match c {
        'O' => Type::Round,
        '#' => Type::Square,
        _ => Type::Empty,
    });
    for i in 0..grid.cells.len() {
        let x = i % grid.width;
        let mut y = (i - x) / grid.width;
        while let Type::Round = grid[(x, y)]
            && y > 0
            && let Type::Empty = grid[(x, y - 1)]
        {
            grid.set_at((x, y - 1), Type::Round);
            grid.set_at((x, y), Type::Empty);
            y -= 1;
        }
    }
    let mut lines_left = grid.cells.chunks_exact(grid.width).len() + 1;
    grid.cells
        .chunks_exact(grid.width)
        .map(|line| {
            lines_left -= 1;
            line.iter()
                .map(|symbol| {
                    if let Type::Round = symbol {
                        lines_left
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut grid = Grid::from_str(input, |((_, _), c)| match c {
        'O' => Type::Round,
        '#' => Type::Square,
        _ => Type::Empty,
    });
    let mut history = Vec::new();
    let mut iteration = 0;
    let mut cycle_found = false;
    while iteration < 1000000000 {
        // north
        for i in 0..grid.cells.len() {
            let x = i % grid.width;
            let mut y = (i - x) / grid.width;
            while let Type::Round = grid[(x, y)]
                && y > 0
                && let Type::Empty = grid[(x, y - 1)]
            {
                grid.set_at((x, y - 1), Type::Round);
                grid.set_at((x, y), Type::Empty);
                y -= 1;
            }
        }

        // west
        for i in 0..grid.cells.len() {
            let mut x = i % grid.width;
            let y = (i - x) / grid.width;
            while let Type::Round = grid[(x, y)]
                && x > 0
                && let Type::Empty = grid[(x - 1, y)]
            {
                grid.set_at((x - 1, y), Type::Round);
                grid.set_at((x, y), Type::Empty);
                x -= 1;
            }
        }

        // south
        for i in 0..grid.cells.len() {
            let j = grid.cells.len() - i - 1;
            let x = j % grid.width;
            let mut y = (j - x) / grid.width;
            while let Type::Round = grid[(x, y)]
                && y < grid.cells.len() / grid.width - 1
                && let Type::Empty = grid[(x, y + 1)]
            {
                grid.set_at((x, y + 1), Type::Round);
                grid.set_at((x, y), Type::Empty);
                y += 1;
            }
        }

        // east
        for i in 0..grid.cells.len() {
            let j = grid.cells.len() - i - 1;
            let mut x = j % grid.width;
            let y = (j - x) / grid.width;
            while let Type::Round = grid[(x, y)]
                && x < grid.width - 1
                && let Type::Empty = grid[(x + 1, y)]
            {
                grid.set_at((x + 1, y), Type::Round);
                grid.set_at((x, y), Type::Empty);
                x += 1;
            }
        }

        if !cycle_found {
            let mut hasher = DefaultHasher::new();
            grid.cells.hash(&mut hasher);
            let hash = hasher.finish();
            if let Some(idx) = history.iter().position(|x| *x == hash) {
                iteration += 1;
                let diff = iteration - idx - 1;
                let cycles_left = 1000000000 - iteration;
                iteration += cycles_left - (cycles_left % diff);
                cycle_found = true;
                continue;
            } else {
                history.push(hash);
            }
        }
        iteration += 1;
    }
    let mut lines_left = grid.cells.chunks_exact(grid.width).len() + 1;
    grid.cells
        .chunks_exact(grid.width)
        .map(|line| {
            lines_left -= 1;
            line.iter()
                .map(|symbol| {
                    if let Type::Round = symbol {
                        lines_left
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT).to_string(), String::from("136"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT).to_string(), String::from("64"))
    }
}
