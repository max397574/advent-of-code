use atoi::atoi;
use bstr::ByteSlice;

fn find_options(total_time: u64, minimal_distance: u64) -> u64 {
    let mut upper = total_time / 2;
    let mut lower = 1;
    let mut time_hold = total_time / 4;
    loop {
        if time_hold * (total_time - time_hold) < minimal_distance {
            lower = time_hold;
            time_hold = (lower + upper) / 2;
        } else if (time_hold - 1) * (total_time - (time_hold - 1)) > minimal_distance {
            upper = time_hold;
            time_hold = (lower + upper) / 2;
        } else {
            break;
        }
    }
    total_time + 1 - (time_hold * 2)
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut lines = input.lines();
    let times = lines.next().unwrap()[11..]
        .split(|&b| b == b' ')
        .filter_map(|block| {
            if block.is_empty() {
                None
            } else {
                Some(atoi::<u64>(block).unwrap())
            }
        });
    let distances = lines.next().unwrap()[11..]
        .split(|&b| b == b' ')
        .filter_map(|block| {
            if block.is_empty() {
                None
            } else {
                Some(atoi::<u64>(block).unwrap())
            }
        });
    let mut total_options = 1;
    for (time, distance) in times.zip(distances) {
        let options = find_options(time, distance);
        if options > 0 {
            total_options *= options;
        }
    }
    total_options
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut time = 0;
    let mut i = 10;
    let mut byte = b'0';
    while byte != b'\n' {
        if byte.is_ascii_digit() {
            time = time * 10 + (byte & 0xf) as u64;
        }
        i += 1;
        byte = input[i];
    }

    i += 11;
    let mut distance = 0;
    while i < input.len() {
        byte = input[i];
        if byte.is_ascii_digit() {
            distance = distance * 10 + (byte & 0xf) as u64;
        }
        i += 1;
    }

    find_options(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "Time:      7  15   30
Distance:  9  40  200";
    const INPUT2: &str = INPUT1;

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1).to_string(), String::from("288"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("71503"))
    }
}
