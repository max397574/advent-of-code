use crate::utils::grid::*;

fn iterate_loop(grid: &Grid<u8>, mut callback: impl FnMut((usize, usize))) {
    let ((x, y), _) = grid.iter().find(|&((_, _), c)| *c == b'S').unwrap();
    let mut x = x as isize;
    let mut y = y as isize;
    let mut dx = 0;
    let mut dy = 0;
    if matches!(grid.get_at_i((x - 1, y)), Some(b'-' | b'L' | b'F')) {
        dx = -1;
    } else if matches!(grid.get_at_i((x + 1, y)), Some(b'-' | b'J' | b'7')) {
        dx = 1;
    } else if matches!(grid.get_at_i((x, y - 1)), Some(b'|' | b'L' | b'J')) {
        dy = -1;
    } else if matches!(grid.get_at_i((x, y + 1)), Some(b'|' | b'F' | b'7')) {
        dy = 1;
    } else {
        panic!();
    }
    loop {
        callback((x as usize, y as usize));
        (x, y) = (x + dx, y + dy);
        (dx, dy) = match grid[(x, y)] {
            b'-' | b'|' => (dx, dy),
            b'L' | b'7' => (dy, dx),
            b'J' | b'F' => (-dy, -dx),
            b'S' => {
                break;
            }
            _ => panic!(),
        };
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let mut distance = 0;
    let grid = Grid::from_str(input, |((_, _), c)| c as u8);
    iterate_loop(&grid, |(_, _)| distance += 1);
    distance / 2
}

fn shoelace(vertices: &[(i32, i32)]) -> f64 {
    let mut sum: i32 = 0;
    let len = vertices.len();
    for i in 0..len - 1 {
        sum += vertices[i].0 * vertices[i + 1].1;
        sum -= vertices[i].1 * vertices[i + 1].0;
    }

    sum += vertices[len - 1].0 * vertices[0].1;
    sum -= vertices[len - 1].1 * vertices[0].0;

    0.5 * (sum as f64).abs()
}

// pub fn part2_picks_shoelace(input: &str) -> impl std::fmt::Display + use<> {
pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let grid = Grid::from_str(input, |((_, _), c)| c as u8);
    let mut vertices = Vec::new();
    iterate_loop(&grid, |(x, y)| vertices.push((x as i32, y as i32)));
    shoelace(&vertices) - (vertices.len() / 2) as f64 + 1.0
}

pub fn _part2_rays(input: &str) -> impl std::fmt::Display + use<> {
    // pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let grid = Grid::from_str(input, |((_, _), c)| c as u8);
    let mut seen = Grid {
        cells: vec![false; grid.cells.len()],
        width: grid.width,
    };
    iterate_loop(&grid, |(x, y)| {
        seen.cells[y * seen.width + x] = true;
    });
    let mut count = 0;
    let mut inside;
    for row in 0..seen.height() {
        inside = false;
        for col in 0..seen.width {
            if *seen.get_at((col, row)).unwrap() {
                // this works because if horizontal segement endpoints both point up inside is
                // switched twice and if they both point down you never switch
                // this is the correct behavior because you just moved along the edge of the shape
                if matches!(grid.get_at((col, row)), Some(b'|' | b'L' | b'J')) {
                    inside = !inside;
                }
            } else if inside {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const INPUT_SIMPLE: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const INPUT_WITH_SQUEEZE: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    const INPUT_BIGGER: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const INPUT_SPARE_PIPES: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1).to_string(), String::from("4"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT_SIMPLE).to_string(), String::from("4"))
    }

    #[test]
    fn part_2_with_squeeze() {
        assert_eq!(part2(INPUT_WITH_SQUEEZE).to_string(), String::from("4"))
    }

    #[test]
    fn part_2_rays() {
        assert_eq!(
            _part2_rays(INPUT_SPARE_PIPES).to_string(),
            String::from("10")
        )
    }

    #[test]
    fn part_2_bigger() {
        assert_eq!(part2(INPUT_BIGGER).to_string(), String::from("8"))
    }

    #[test]
    fn part_2_with_spare_pipes() {
        assert_eq!(part2(INPUT_SPARE_PIPES).to_string(), String::from("10"))
    }
}
