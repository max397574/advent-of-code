use atoi::atoi;
use bstr::ByteSlice;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut lines = input.lines();
    let mut curr_state = lines.next().unwrap()[7..]
        .split(|&b| b == b' ')
        .map(|num_str| atoi::<u64>(num_str).unwrap())
        .collect::<Vec<_>>();
    lines.next();
    let mut maps: Vec<(u64, u64, u64)> = Vec::new();

    for line in lines {
        if line.is_empty() {
            for item in curr_state.iter_mut() {
                for &(dest, src, len) in &maps {
                    if *item >= src && item < &mut (src + len) {
                        *item = *item - src + dest;
                        break;
                    }
                }
            }
            maps = Vec::new();
            continue;
        }

        if !line[0].is_ascii_digit() {
            continue;
        }

        let mut numbers = line.split(|&b| b == b' ');
        let dest = atoi::<u64>(numbers.next().unwrap()).unwrap();
        let src = atoi::<u64>(numbers.next().unwrap()).unwrap();
        let len = atoi::<u64>(numbers.next().unwrap()).unwrap();
        maps.push((dest, src, len));
    }
    for item in curr_state.iter_mut() {
        for &(dest, src, len) in &maps {
            if *item > src && item < &mut (src + len) {
                *item = *item - src + dest;
                break;
            }
        }
    }
    let mut smallest = curr_state[0];
    for item in curr_state.iter() {
        smallest = smallest.min(*item);
    }
    smallest
}

// TODO: for real input gave result one too high, figure out why
pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut lines = input.lines();
    let numbers = lines.next().unwrap()[7..]
        .split(|&b| b == b' ')
        .map(|num_str| atoi::<u64>(num_str).unwrap())
        .collect::<Vec<_>>();
    let mut curr_state = Vec::new();
    for number_pair in numbers.iter().array_chunks::<2>() {
        for number in *number_pair[0]..(number_pair[0] + number_pair[1]) {
            curr_state.push(number);
        }
    }

    lines.next();
    let mut maps: Vec<(u64, u64, u64)> = Vec::new();

    for line in lines {
        if line.is_empty() {
            for item in curr_state.iter_mut() {
                for &(dest, src, len) in &maps {
                    if *item >= src && item <= &mut (src + len) {
                        *item = *item - src + dest;
                        break;
                    }
                }
            }
            maps = Vec::new();
            continue;
        }

        if !line[0].is_ascii_digit() {
            continue;
        }

        let mut numbers = line.split(|&b| b == b' ');
        let dest = atoi::<u64>(numbers.next().unwrap()).unwrap();
        let src = atoi::<u64>(numbers.next().unwrap()).unwrap();
        let len = atoi::<u64>(numbers.next().unwrap()).unwrap();
        maps.push((dest, src, len));
    }
    for item in curr_state.iter_mut() {
        for &(dest, src, len) in &maps {
            if *item > src && item < &mut (src + len) {
                *item = *item - src + dest;
                break;
            }
        }
    }
    let mut smallest = curr_state[0];
    for item in curr_state.iter() {
        smallest = smallest.min(*item);
    }
    smallest
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    const INPUT2: &str = INPUT1;

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1).to_string(), String::from("35"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("46"))
    }
}
