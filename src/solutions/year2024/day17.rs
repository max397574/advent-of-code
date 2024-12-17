use std::{hint::unreachable_unchecked, intrinsics::unchecked_sub};

#[inline(always)]
fn swar_parsing(n: u64) -> u32 {
    use std::num::Wrapping as W;

    let mut n = W(n);
    let mask = W(0xFF | (0xFF << 32));
    let mul1 = W(100 + (1000000 << 32));
    let mul2 = W(1 + (10000 << 32));

    n -= W(u64::from_ne_bytes([b'0'; 8]));
    n = (n * W(10)) + (n >> 8);
    n = (((n & mask) * mul1) + (((n >> 16) & mask) * mul2)) >> 32;

    n.0 as u32
}

static mut OUTPUT: [u8; 17] = [
    0, b',', 0, b',', 0, b',', 0, b',', 0, b',', 0, b',', 0, b',', 0, b',', 0,
];

pub fn part1(input: &str) -> &str {
    unsafe {
        let input = input.as_bytes();
        let mut out_vals = 0;
        let mut input = input.as_ptr();
        input = input.add("Register A: ".len());
        let mut reg_a = swar_parsing(*(input as *const u64));
        let mut reg_b = 0;
        let mut reg_c = 0;
        input = input.add("\nRegister B: 0\nRegister C: 0\n\nProgram: ".len() + 8);
        let mut programm = [0; 16];
        let mut i = 0;
        while i < 16 {
            *programm.get_unchecked_mut(i) = unchecked_sub(*input.offset(0), b'0');
            input = input.add(2);
            i += 1;
        }
        let mut ip = 0;
        while ip < programm.len() {
            let instruction = *programm.get_unchecked(ip);
            let operand = *programm.get_unchecked(ip + 1);
            let combo_operand = || match operand {
                0..=3 => operand as u32,
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                _ => unreachable_unchecked(),
            };
            match instruction {
                0 => reg_a >>= combo_operand(),
                1 => reg_b ^= *programm.get_unchecked(ip + 1) as u32,
                2 => reg_b = combo_operand() % 8,
                3 => {
                    if reg_a != 0 {
                        ip = *programm.get_unchecked(ip + 1) as usize;
                        continue;
                    }
                }
                4 => reg_b ^= reg_c,
                5 => {
                    *OUTPUT.get_unchecked_mut(out_vals * 2) =
                        (combo_operand() & 0b111) as u8 + b'0';
                    //OUTPUT[out_vals * 2] = (combo_operand() & 0b111) as u8;
                    out_vals += 1;
                }
                6 => reg_b = reg_a >> combo_operand(),
                7 => reg_c = reg_a >> combo_operand(),
                _ => unreachable_unchecked(),
            }
            ip += 2;
        }
        std::str::from_utf8_unchecked(&OUTPUT)
    }
}

pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    unsafe {
        let len = input.len();
        let mut input = input.as_ptr().add(len - 1);
        if *input.offset(0) == b'\n' {
            input = input.sub(1);
        }
        let mut program = [0; 16];
        for i in 0..16 {
            program[15 - i] = *input.offset(0) as u64 - b'0' as u64;
            input = input.sub(2);
        }
        let mut a = 0;
        let mut out = Vec::new();
        loop {
            out.clear();
            let mut reg_a = a;
            let mut reg_b = 0;
            let mut reg_c = 0;
            let mut ip = 0;
            while ip < program.len() {
                let instruction = program[ip];
                let operand = program[ip + 1];
                let combo_operand = || match operand {
                    0..=3 => operand,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => unreachable!(),
                };
                match instruction {
                    0 => reg_a >>= combo_operand(),
                    1 => reg_b ^= program[ip + 1],
                    2 => reg_b = combo_operand() % 8,
                    3 => {
                        if reg_a != 0 {
                            ip = program[ip + 1] as usize;
                            continue;
                        }
                    }
                    4 => reg_b ^= reg_c,
                    5 => {
                        out.push(combo_operand() & 0b111);
                    }
                    6 => reg_b = reg_a >> combo_operand(),
                    7 => reg_c = reg_a >> combo_operand(),
                    _ => unreachable!(),
                }
                ip += 2;
            }
            if program.ends_with(&out) {
                if out.len() == 16 {
                    return a;
                } else {
                    a <<= 3;
                }
            } else {
                while a & 0b111 == 0b111 {
                    a >>= 3;
                }
                a += 1;
            }
        }
    }
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
    fn part_1() {
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
