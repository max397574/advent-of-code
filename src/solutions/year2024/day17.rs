use bstr::ByteSlice;

use crate::utils::parsing::ByteParsing;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut lines = input.lines();
    let mut reg_a: u32 = lines.next().unwrap()[12..].as_num();
    let mut reg_b: u32 = lines.next().unwrap()[12..].as_num();
    let mut reg_c: u32 = lines.next().unwrap()[12..].as_num();
    lines.next();
    let programm: Vec<u8> = lines.next().unwrap()[9..]
        .split(|&c| c == b',')
        .map(|n| n[0] - b'0')
        .collect();
    let mut outputs = Vec::new();
    let mut ip = 0;
    while ip < programm.len() {
        let instruction = programm[ip];
        let operand = programm[ip + 1];
        let combo_operand = || match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => unreachable!(),
        };
        match instruction {
            0 => {
                reg_a >>= combo_operand();
                ip += 2;
            }
            1 => {
                reg_b ^= programm[ip + 1] as u32;
                ip += 2;
            }
            2 => {
                reg_b = combo_operand() % 8;
                ip += 2;
            }
            3 => {
                if reg_a != 0 {
                    ip = programm[ip + 1] as usize;
                } else {
                    ip += 2;
                }
            }
            4 => {
                reg_b ^= reg_c;
                ip += 2;
            }
            5 => {
                outputs.push((combo_operand() % 8).to_string());
                ip += 2;
            }
            6 => {
                reg_b = reg_a >> combo_operand();
                ip += 2;
            }
            7 => {
                reg_c = reg_a >> combo_operand();
                ip += 2;
            }
            _ => unreachable!(),
        }
    }
    outputs.join(",")
}

// Step 1: read what your code does
// Step 2: notice what happens to A
// Step 3: notice how that influences B
// Step 4: notice that only the nth (from the right) 3-bit group and all the bits to its left influence the nth outputted number
// Step 5: bruteforce each 3 bit group separately, starting from the leftmost one (i.e. the one generating the last outputted number)
pub fn part2(_input: &str) -> impl std::fmt::Display {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part1_() {
        assert_eq!(
            part1(INPUT).to_string(),
            String::from("4,6,3,5,6,3,5,2,1,0")
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("117440"))
    }
}
