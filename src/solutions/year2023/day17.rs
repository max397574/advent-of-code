use crate::utils::graphs::dijkstra;
use crate::utils::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Node {
    forward_count: u8,
    pos: (usize, usize),
    direction: Direction,
}

fn filter_oob(coords: Vec<Node>, dimensions: (usize, usize)) -> Vec<Node> {
    coords
        .into_iter()
        .filter(|node| {
            let (x, y) = node.pos;
            x < dimensions.0 && y < dimensions.1
        })
        .collect()
}

pub fn solve(input: &str, min_straight: u8, max_straight: u8) -> impl std::fmt::Display + use<> {
    let grid = Grid::from_str(input, |((_, _), c)| (c as u8 - 48) as usize);
    let dimensions = grid.get_dimensions();

    let adjacency = |node: Node| {
        let (x, y) = node.pos;

        let mut new_nodes = Vec::new();

        let try_push =
            |nodes: &mut Vec<Node>, nx: usize, ny: usize, fwd_cnt: u8, dir: Direction| {
                nodes.push(Node {
                    pos: (nx, ny),
                    forward_count: fwd_cnt,
                    direction: dir,
                });
            };

        let forward_step = match node.direction {
            Direction::Up => (0isize, -1isize),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let turn_dirs = match node.direction {
            Direction::Up | Direction::Down => vec![
                (Direction::Left, (-1isize, 0isize)),
                (Direction::Right, (1, 0)),
            ],
            Direction::Left | Direction::Right => {
                vec![(Direction::Up, (0, -1)), (Direction::Down, (0, 1))]
            }
        };

        if node.forward_count + 1 >= min_straight {
            for (tdir, (dx, dy)) in turn_dirs {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 {
                    try_push(&mut new_nodes, nx as usize, ny as usize, 0, tdir);
                }
            }
        }

        if node.forward_count < max_straight - 1 {
            let nx = x as isize + forward_step.0;
            let ny = y as isize + forward_step.1;
            if nx >= 0 && ny >= 0 {
                try_push(
                    &mut new_nodes,
                    nx as usize,
                    ny as usize,
                    node.forward_count + 1,
                    node.direction,
                );
            }
        }

        filter_oob(new_nodes, dimensions)
    };

    let last = (dimensions.0 - 1, dimensions.1 - 1);
    dijkstra::shortest_path(
        Node {
            forward_count: 0,
            pos: (0, 0),
            direction: Direction::Right,
        },
        &adjacency,
        &|node: Node| grid[node.pos],
        &|node: Node| node.pos == last,
    )
    .unwrap()
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    solve(input, 0, 3)
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    solve(input, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("102"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("94"))
    }
}
