#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum State {
    Empty,
    File(u32),
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let input: Vec<u32> = input.trim().bytes().map(|b| (b - b'0') as u32).collect();
    let mut id = 0;
    let mut is_file = true;
    let mut state = Vec::new();
    input.iter().for_each(|&i| {
        if is_file {
            is_file = false;
            for _ in 0..i {
                state.push(State::File(id));
            }
            id += 1;
        } else {
            is_file = true;
            for _ in 0..i {
                state.push(State::Empty);
            }
        }
    });

    let mut first_space = state.iter().position(|val| *val == State::Empty).unwrap();
    let mut last = state.len()
        - 1
        - state
            .iter()
            .rev()
            .position(|val| matches!(*val, State::File(_)))
            .unwrap();
    while first_space < last {
        let ele = state[last];
        state[first_space] = ele;
        state[last] = State::Empty;
        first_space = state.iter().position(|val| *val == State::Empty).unwrap();
        last = state.len()
            - 1
            - state
                .iter()
                .rev()
                .position(|val| *val != State::Empty)
                .unwrap();
    }
    let mut sum = 0;
    (0..first_space).for_each(|i| {
        if let State::File(id) = state[i] {
            sum += i * id as usize;
        }
    });
    sum
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    // taken from https://github.com/SkiFire13/adventofcode-2024-rs/commit/20b7ffe3956e7288917d18f4284d4d5b64ad1ae8
    let input: Vec<u8> = input.trim().bytes().map(|b| b - b'0').collect();
    let mut pos = 0;
    let mut poss = Vec::new();
    for in_char in &input {
        poss.push(pos);
        pos += *in_char as usize;
    }

    let mut tot = 0;

    for i in (0..(input.len()).div_ceil(2)).rev() {
        if let Some(j) =
            (0..i).find(|&j| poss[2 + 2 * j] - poss[1 + 2 * j] >= input[2 * i] as usize)
        {
            let pos = poss[1 + 2 * j];
            let len = input[2 * i] as usize;
            let new_pos = pos + len;
            tot += i * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));
            poss[1 + 2 * j] += len;
        } else {
            let pos = poss[2 * i];
            let len = input[2 * i] as usize;
            let new_pos = pos + len;
            tot += i * ((new_pos * (new_pos - 1) / 2) - (pos * (pos.saturating_sub(1)) / 2));
        }
    }

    tot
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("1928"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("2858"))
    }
}
