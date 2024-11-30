use std::cmp::max;

/// Returns true if the sand did land
fn drop_sand(grid: &mut [[bool; 1000]; 1000], max_y: usize, ignore_depth: bool) -> bool {
    let mut x: usize = 500;
    let mut y: usize = 0;
    let mut did_move = true;
    while did_move {
        if y > max_y && !ignore_depth {
            return false;
        }
        if grid[x][y + 1] {
            y += 1;
        } else if grid[x - 1][y + 1] {
            x -= 1;
            y += 1;
        } else if grid[x + 1][y + 1] {
            x += 1;
            y += 1;
        } else {
            did_move = false;
        }
        if !did_move {
            grid[x][y] = false;
            if x == 500 && y == 0 {
                return false;
            }
        }
    }
    true
}

fn get_grid(input: &str) -> (Box<[[bool; 1000]; 1000]>, usize) {
    let mut max_y = 0;
    let mut grid = Box::new([[true; 1000]; 1000]);
    for line in input.lines() {
        let _ = line
            .split("->")
            .map_windows(|[start, end]| {
                let [x1, y1] = start
                    .trim()
                    .split(',')
                    .map(|coord| coord.parse::<usize>().unwrap())
                    .next_chunk()
                    .unwrap();
                let [x2, y2] = end
                    .trim()
                    .split(',')
                    .map(|coord| coord.parse::<usize>().unwrap())
                    .next_chunk()
                    .unwrap();
                max_y = max(max_y, max(y1, y2));
                if x1 == x2 {
                    for y in y1.min(y2)..=y2.max(y1) {
                        grid[x1][y] = false;
                    }
                } else {
                    (x1.min(x2)..=x2.max(x1)).for_each(|x| {
                        grid[x][y1] = false;
                    });
                }
            })
            .collect::<Vec<_>>();
    }
    (grid, max_y)
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sands_dropped = 0;
    let (mut grid, max_y) = get_grid(input);

    loop {
        if !drop_sand(&mut grid, max_y, false) {
            return sands_dropped;
        } else {
            sands_dropped += 1;
        }
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut sands_dropped = 0;
    let (mut grid, max_y) = get_grid(input);
    (0..1000).for_each(|x| {
        grid[x][max_y + 2] = false;
    });
    loop {
        if !drop_sand(&mut grid, max_y + 2, true) {
            return sands_dropped + 1;
        } else {
            sands_dropped += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("24"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("93"))
    }
}
