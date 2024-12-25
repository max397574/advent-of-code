#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum State {
    Empty,
    File(u32),
}

pub fn part1(input: &str) -> impl std::fmt::Display {
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

pub fn part2(_input: &str) -> impl std::fmt::Display {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("1928"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(INPUT).to_string(), String::from("2858"))
    }
}
