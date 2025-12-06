pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let width = input.find('\n').unwrap();
    let mut pos = input.find('S').unwrap();
    let height = (input.len() + 1) / (width + 1);
    let input = input.as_bytes();

    let mut visited: Vec<bool> = vec![false; (width + 1) * height];
    let mut count = 0;
    let mut stack: Vec<usize> = Vec::new();
    stack.push(pos);
    while !stack.is_empty() {
        pos = stack.pop().unwrap();
        if !visited[pos] {
            visited[pos] = true;
            match input[pos] {
                b'.' => {
                    let new = pos + width + 1;
                    if new < input.len() {
                        stack.push(new);
                    }
                }
                _ => {
                    let x = pos % (width + 1);
                    count += 1;
                    if x > 0 {
                        stack.push(pos - 1);
                    }
                    if x < width {
                        stack.push(pos + 1);
                    }
                }
            }
        }
    }
    count
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let width = input.find('\n').unwrap();
    let start = input.find('S').unwrap();
    let height = (input.len() + 1) / (width + 1);
    let input = input.as_bytes();

    let mut possible_timelines: Vec<u64> = vec![0; (width + 1) * height];
    possible_timelines[start] = 1;
    unsafe {
        for y in 1..height {
            for x in 0..width {
                let pos = x + y * (width + 1);
                let above = pos - (width + 1);
                let above_val = *possible_timelines.get_unchecked(above);
                if above_val == 0 {
                    continue;
                }
                match *input.get_unchecked(pos) {
                    b'^' => {
                        if x > 0 {
                            *possible_timelines.get_unchecked_mut(pos - 1) += above_val;
                        }
                        if x < width {
                            *possible_timelines.get_unchecked_mut(pos + 1) += above_val;
                        }
                    }
                    _ => {
                        *possible_timelines.get_unchecked_mut(pos) += above_val;
                    }
                }
            }
        }
    }
    // for y in 0..height {
    //     println!(
    //         "{:?}",
    //         &possible_timelines[y * (width + 1)..y * (width + 1) + width + 1]
    //     );
    // }
    possible_timelines[(width + 1) * (height - 1)..]
        .iter()
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............\n";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("21"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("40"))
    }
}
