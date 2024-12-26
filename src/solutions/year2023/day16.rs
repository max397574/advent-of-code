use std::collections::VecDeque;

pub fn do_thing(
    input: &[u8],
    start_pos: usize,
    start_direction: u8,
    width: usize,
    height: usize,
) -> i32 {
    // whether a tile has been energized
    let mut energized: Vec<bool> = vec![false; (width + 1) * height];
    // cache for where beam have been
    // use bitflags: 0 -> up, 1 -> right, 2 -> down, 3 -> left
    let mut cache: Vec<u8> = vec![0; (width + 1) * height];
    let mut energized_count = 0;
    let mut direction = start_direction;
    let mut pos = start_pos;
    let mut queue: VecDeque<(usize, u8)> = VecDeque::new();
    // queue.push_front((0, 1));

    match input[pos] {
        b'/' => {
            direction = [1, 0, 3, 2][direction as usize];
        }
        b'\\' => {
            direction = 3 - direction;
        }
        b'|' => match direction {
            0 | 2 => {}
            // 1 or 3
            _ => {
                direction = 0;
                queue.push_back((pos, 2));
            }
        },
        b'-' => match direction {
            1 | 3 => {}
            // 0 or 2
            _ => {
                direction = 1;
                queue.push_back((pos, 3));
            }
        },
        b'.' => {}
        _ => unreachable!(),
    }
    queue.push_back((pos, direction));

    fn exec_position(
        direction: u8,
        pos: usize,
        width: usize,
        height: usize,
        queue: &mut VecDeque<(usize, u8)>,
        input: &[u8],
    ) {
        let (x, y) = (pos % (width + 1), pos / (width + 1));
        let mut direction = direction;
        let mut pos = pos;
        // println!("({x},{y}), {direction}");

        let invalid = (direction == 0 && y == 0)
            || (direction == 1 && x == width - 1)
            || (direction == 2 && y == height - 1)
            || (direction == 3 && x == 0);

        if invalid {
            return;
        }

        match direction {
            0 => pos -= width + 1,
            1 => pos += 1,
            2 => pos += width + 1,
            3 => pos -= 1,
            _ => unreachable!(),
        }
        match input[pos] {
            b'/' => {
                direction = [1, 0, 3, 2][direction as usize];
            }
            b'\\' => {
                direction = 3 - direction;
            }
            b'|' => match direction {
                0 | 2 => {}
                // 1 or 3
                _ => {
                    direction = 0;
                    queue.push_back((pos, 2));
                }
            },
            b'-' => match direction {
                1 | 3 => {}
                // 0 or 2
                _ => {
                    direction = 1;
                    queue.push_back((pos, 3));
                }
            },
            b'.' => {}
            _ => unreachable!(),
        }
        queue.push_back((pos, direction));
    }
    while !queue.is_empty() {
        (pos, direction) = queue.pop_front().unwrap();

        if (cache[pos] & 1 << direction) == 0 {
            cache[pos] |= 1 << direction;

            if !energized[pos] {
                energized_count += 1;
                energized[pos] = true;
            }

            exec_position(direction, pos, width, height, &mut queue, input);
        }
    }
    energized_count
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let width = input.find('\n').unwrap();
    let height = (input.len() + 1) / (width + 1);
    let input = input.as_bytes();

    do_thing(input, 0, 1, width, height)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let width = input.find('\n').unwrap();
    let height = (input.len() + 1) / (width + 1);
    let input = input.as_bytes();

    let mut max = 0;
    for x in 0..width {
        max = max.max(do_thing(input, x, 2, width, height));
        max = max.max(do_thing(
            input,
            x + (height - 1) * (width + 1),
            0,
            width,
            height,
        ));
    }

    for y in 0..height {
        max = max.max(do_thing(input, y * (width + 1), 1, width, height));
        max = max.max(do_thing(
            input,
            y * (width + 1) + width - 1,
            3,
            width,
            height,
        ));
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("46"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("51"))
    }
}
