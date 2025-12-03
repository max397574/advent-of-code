use crate::utils::grid::Grid;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let grid = Grid::from_str(input, |((_, _), c)| c);
    let mut removable = 0;
    grid.iter().for_each(|((x, y), c)| {
        if *c == '@' {
            let mut roll_count = 0;
            for neighbour in grid.all_neighbours((x, y)) {
                if *grid.get(neighbour).unwrap() == '@' {
                    roll_count += 1;
                }
            }
            if roll_count < 4 {
                removable += 1;
            }
        }
    });
    removable
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let mut grid = Grid::from_str(input, |((_, _), c)| c);
    let mut to_remove = Vec::new();
    let mut removable = 0;
    loop {
        grid.iter().for_each(|((x, y), c)| {
            if *c == '@' {
                let mut roll_count = 0;
                for neighbour in grid.all_neighbours((x, y)) {
                    if *grid.get(neighbour).unwrap() == '@' {
                        roll_count += 1;
                    }
                }
                if roll_count < 4 {
                    to_remove.push((x, y));
                }
            }
        });
        if to_remove.is_empty() {
            break;
        } else {
            for pos in &to_remove {
                grid.set_at(*pos, '.');
            }
            removable += to_remove.len();
            to_remove.clear();
        }
    }
    removable
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.\n";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("13"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("43"))
    }
}
