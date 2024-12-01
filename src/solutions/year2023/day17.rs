//use crate::utils::graphs::dijkstra;
//use crate::utils::grid::Grid;
//
//#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
//pub enum Direction {
//    Up,
//    Down,
//    Left,
//    Right,
//}
//
//#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
//pub struct Node {
//    forward_count: u8,
//    pos: (usize, usize),
//    direction: Direction,
//}
//
//fn filter_oob(coords: Vec<Node>, dimensions: (usize, usize)) -> Vec<Node> {
//    coords
//        .iter()
//        .filter(|node| {
//            let (x, y) = node.pos;
//            !(x >= dimensions.0 || y >= dimensions.1)
//        })
//        .copied()
//        .collect()
//}

pub fn part1(_input: &str) -> impl std::fmt::Display {
    //let grid = Grid::from_str(input, |((_, _), c)| (c as u8 - 48) as usize);
    //let dimensions = grid.get_dimensions();
    //let adjacency = |node: Node| {
    //let (x, y) = node.pos;
    //filter_oob(
    //    match node.direction {
    //        Direction::Up => {
    //            if node.forward_count < 3 {
    //                let mut nodes = vec![Node { pos }];
    //                if y > 0 {
    //                    nodes.push(Node {
    //                        pos: (x, y - 1),
    //                        forward_count: node.forward_count + 1,
    //                        direction: Direction::Up,
    //                    })
    //                }
    //                nodes
    //            } else {
    //                vec![]
    //            }
    //        }
    //        Direction::Down => {
    //            vec![]
    //        }
    //        Direction::Left => {
    //            vec![]
    //        }
    //        Direction::Right => {
    //            vec![]
    //        }
    //    },
    //    dimensions,
    //)
    //};
    //let last = (dimensions.0 - 1, dimensions.1 - 1);
    //dijkstra::shortest_path(
    //    Node {
    //        forward_count: 1,
    //        pos: (0, 0),
    //        direction: Direction::Right,
    //    },
    //    &adjacency,
    //    &|node: Node| grid[node.pos],
    //    &|node: Node| node.pos == last,
    //)
    //.unwrap()
    0
}

pub fn part2(_input: &str) -> impl std::fmt::Display {
    0
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
        assert_eq!(part1(INPUT).to_string(), String::from("0"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(INPUT).to_string(), String::from("0"))
    }
}
