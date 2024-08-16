use bstr::ByteSlice;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let total_amount = input.lines().count();
    let mut one_counts: Vec<u32> = vec![0; input.lines().next().unwrap().len()];
    input.lines().for_each(|line| {
        line.bytes().enumerate().for_each(|(idx, char)| {
            if char == b'1' {
                one_counts[idx] += 1;
            }
        })
    });
    let mut gamma = String::new();
    let mut epsilon = String::new();
    one_counts.iter().for_each(|count| {
        if *count > (total_amount / 2) as u32 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    });
    u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut input1 = input
        .as_bytes()
        .split(|b| b == &b'\n')
        .collect::<Vec<_>>()
        .clone();
    let mut input2 = input
        .as_bytes()
        .split(|b| b == &b'\n')
        .collect::<Vec<_>>()
        .clone();
    if input1.last().unwrap().is_empty() {
        input1.pop();
        input2.pop();
    }

    let len = input1.first().unwrap().len();
    for i in 0..len {
        let mut ones = 0;
        let mut zeros = 0;
        for line in input1.clone() {
            if line[i] == b'1' {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        input1.retain(|line| {
            if ones > zeros || zeros == ones {
                line[i] == b'1'
            } else {
                line[i] == b'0'
            }
        });
        if input1.len() == 1 {
            break;
        }
    }
    for i in 0..len {
        let mut ones = 0;
        let mut zeros = 0;
        for line in input2.clone() {
            if line[i] == b'1' {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        println!("{zeros}, {ones}");
        input2.retain(|line| {
            if ones > zeros || zeros == ones {
                line[i] == b'0'
            } else {
                line[i] == b'1'
            }
        });
        if input2.len() == 1 {
            break;
        }
    }
    println!(
        "{}, {}",
        std::str::from_utf8(input1[0]).unwrap(),
        std::str::from_utf8(input2[0]).unwrap()
    );

    return u32::from_str_radix(std::str::from_utf8(input1[0]).unwrap(), 2).unwrap()
        * u32::from_str_radix(std::str::from_utf8(input2[0]).unwrap(), 2).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT).to_string(), String::from("198"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT).to_string(), String::from("230"))
    }
}
