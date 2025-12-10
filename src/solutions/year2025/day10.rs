use std::collections::VecDeque;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let mut queue = VecDeque::with_capacity(1000);
    unsafe {
        input
            .lines()
            .map(|line| {
                let input = line.as_bytes();
                let mut input = input.as_ptr();
                input = input.add(1);
                let mut i = 0;
                let mut start = 0_u16;
                while *input != b']' {
                    start |= (*input as u16 & 1) << i;
                    i += 1;
                    input = input.add(1);
                }
                input = input.add(3);
                let mut buttons = [0; 16];
                let mut mask = 0;
                let mut button_idx = 0;
                while *input != b'{' {
                    if *input == b' ' {
                        *buttons.get_unchecked_mut(button_idx) = mask;
                        button_idx += 1;
                        if *input.offset(1) == b'{' {
                            break;
                        }
                        input = input.add(2);
                        mask = 0;
                    }
                    mask |= 1 << (*input - b'0');
                    input = input.add(2);
                }

                let mut seen = [0u128; 8];

                let mut result = 0;
                queue.clear();
                queue.push_back((0, start));
                while !queue.is_empty() {
                    let next = queue.pop_front().unwrap_unchecked();

                    let state = next.1 as usize;

                    let idx = state >> 7;
                    let bitpos = state & 127;
                    let bit = 1u128 << bitpos;

                    if (seen.get_unchecked(idx) & bit) != 0 {
                        continue;
                    }

                    if next.1 == 0 {
                        result = next.0;
                        break;
                    }

                    *seen.get_unchecked_mut(idx) |= bit;

                    for i in 0..button_idx {
                        let button = *buttons.get_unchecked(i);
                        let new_state = next.1 ^ button;
                        let new = (next.0 + 1, new_state);

                        let idx = (new_state as usize) >> 7;
                        let bitpos = (new_state as usize) & 127;
                        let bit = 1u128 << bitpos;

                        if (seen.get_unchecked(idx) & bit) == 0 {
                            queue.push_back(new);
                        }
                    }
                }
                result
            })
            .sum::<u32>()
    }
}

pub fn part2(_input: &str) -> impl std::fmt::Display + use<> {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("7"))
    }

    // #[test]
    fn _part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("0"))
    }
}
