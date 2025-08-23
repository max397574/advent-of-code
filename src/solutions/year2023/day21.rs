use std::collections::VecDeque;

pub fn bfs_reachable(input: &str, steps: usize, wrap: bool) -> impl std::fmt::Display {
    let width = input.find('\n').unwrap();
    let height = (input.len() + 1) / (width + 1);

    let mut adjacency_list = vec![vec![]; width * height];
    let input = input.as_bytes();

    let mut start = 0;

    for x in 0..width {
        for y in 0..height {
            if input[y * (width + 1) + x] == b'.' || input[y * (width + 1) + x] == b'S' {
                if !wrap {
                    let mut potential_list = Vec::new();
                    if y > 0 {
                        potential_list.push((x, y - 1));
                    }
                    if x > 0 {
                        potential_list.push((x - 1, y));
                    }
                    if x < width - 1 {
                        potential_list.push((x + 1, y));
                    }
                    if y < height - 1 {
                        potential_list.push((x, y + 1));
                    }
                    potential_list
                        .iter()
                        .filter(|(tmp_x, tmp_y)| {
                            input[tmp_y * (width + 1) + tmp_x] == b'.'
                                || input[tmp_y * (width + 1) + tmp_x] == b'S'
                        })
                        .for_each(|(tmp_x, tmp_y)| {
                            adjacency_list[y * width + x].push(tmp_y * width + tmp_x);
                        });
                } else {
                    let potential_list = [
                        (x, (y + 1) % height),
                        (x, (y + height - 1) % height),
                        ((x + 1) % width, y),
                        ((x + width - 1) % width, y),
                    ];
                    potential_list
                        .iter()
                        .filter(|(tmp_x, tmp_y)| {
                            input[tmp_y * (width + 1) + tmp_x] == b'.'
                                || input[tmp_y * (width + 1) + tmp_x] == b'S'
                        })
                        .for_each(|(tmp_x, tmp_y)| {
                            adjacency_list[y * width + x].push(tmp_y * width + tmp_x);
                        });
                }
            }
            if input[y * (width + 1) + x] == b'S' {
                start = y * width + x;
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, start));
    let mut visited = vec![false; width * height];
    let mut count = 0;
    while let Some((level, pos)) = queue.pop_front() {
        if level == steps {
            if !visited[pos] {
                count += 1;
                visited[pos] = true;
            }
            continue;
        }
        adjacency_list[pos].iter().for_each(|neighbour| {
            if !queue.contains(&(level + 1, *neighbour)) {
                queue.push_back((level + 1, *neighbour));
            }
        });
    }

    count
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    bfs_reachable(input, 64, false)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    // doesn't work because of visiting the same block multiple itmes in a different "clone" when
    // wrapping around isn't counted
    bfs_reachable(input, 26501365, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    // const INPUT: &str = "";
    const _INPUT2: &str = "";

    #[test]
    fn part1() {
        assert_eq!(
            bfs_reachable(INPUT, 6, false).to_string(),
            String::from("16")
        )
    }

    // #[test]
    fn _part2() {
        assert_eq!(
            bfs_reachable(INPUT, 5000, true).to_string(),
            String::from("16733044")
        )
    }
}
