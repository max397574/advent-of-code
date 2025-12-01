#[allow(clippy::missing_safety_doc)]
pub unsafe fn do_thing(
    input: &[u8],
    start_pos: usize,
    start_direction: u8,
    width: usize,
    height: usize,
) -> i32 {
    unsafe {
        // whether a tile has been energized
        let mut energized: [u128; 128] = [0; 128];
        // cache for where beam have been
        // use bitflags: 0 -> up, 1 -> right, 2 -> down, 3 -> left
        let mut cache: Vec<u8> = vec![0; (width + 1) * height];
        let mut energized_count = 0;
        let mut direction = start_direction;
        let mut pos = start_pos;
        let mut stack: Vec<usize> = Vec::new();
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
                    let val = (pos << 4) | (1 << 2);
                    stack.push(val);
                }
            },
            b'-' => match direction {
                1 | 3 => {}
                // 0 or 2
                _ => {
                    direction = 1;
                    let val = (pos << 4) | (1 << 3);
                    stack.push(val);
                }
            },
            _ => {}
        }
        let val = (pos << 4) | (1 << direction);
        stack.push(val);

        unsafe fn exec_position(
            direction: u8,
            pos: usize,
            width: usize,
            height: usize,
            queue: &mut Vec<usize>,
            input: &[u8],
        ) {
            unsafe {
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
                match input.get_unchecked(pos) {
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
                            let val = (pos << 4) | (1 << 2);
                            queue.push(val);
                        }
                    },
                    b'-' => match direction {
                        1 | 3 => {}
                        // 0 or 2
                        _ => {
                            direction = 1;
                            let val = (pos << 4) | (1 << 3);
                            queue.push(val);
                        }
                    },
                    b'.' => {}
                    _ => unreachable!(),
                }
                let val = (pos << 4) | (1 << direction);
                queue.push(val);
            }
        }
        while let Some(val) = stack.pop() {
            pos = val >> 4;
            let direction = val.trailing_zeros() as u8;

            if (cache.get_unchecked(pos) & 1 << direction) == 0 {
                *cache.get_unchecked_mut(pos) |= 1 << direction;

                if energized.get_unchecked(pos >> 7) & (1 << (pos % 128)) == 0 {
                    energized_count += 1;
                    *energized.get_unchecked_mut(pos >> 7) |= 1 << (pos % 128);
                }

                exec_position(direction, pos, width, height, &mut stack, input);
            }
        }
        energized_count
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let width = input.find('\n').unwrap();
    let height = (input.len() + 1) / (width + 1);
    let input = input.as_bytes();

    unsafe { do_thing(input, 0, 1, width, height) }
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let width = input.find('\n').unwrap();
    let height = (input.len() + 1) / (width + 1);
    let input = input.as_bytes();

    let mut max = 0;
    unsafe {
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
