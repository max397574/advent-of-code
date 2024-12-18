#![feature(core_intrinsics)]
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

        let mut reg_a = swar_parsing(input.cast::<u64>().read_unaligned());
        let mut reg_b = 0;
        let mut reg_c = 0;
        input = input.add("\nRegister B: 0\nRegister C: 0\n\nProgram: ".len() + 8);
        let mut program = [0; 16];
        let mut i = 0;
        while i < 16 {
            *program.get_unchecked_mut(i) = unchecked_sub(*input.offset(0), b'0');
            input = input.add(2);
            i += 1;
        }
        loop {
            reg_b = reg_a % 8;

            reg_b ^= program[3] as u32;

            reg_c = reg_a >> reg_b;

            if program[6] == 1 {
                reg_b ^= program[7] as u32;
            } else if program[6] == 4 {
                reg_b ^= reg_c;
            } else if program[6] == 0 {
                reg_a >>= 3;
            }

            if program[8] == 4 {
                reg_b ^= reg_c;
            } else if program[8] == 0 {
                reg_a >>= 3;
            } else if program[8] == 1 {
                reg_b ^= program[9] as u32;
            }

            if program[10] == 5 {
                *OUTPUT.get_unchecked_mut(out_vals * 2) = (reg_b & 0b111) as u8 + b'0';
                out_vals += 1;
                reg_a >>= 3;
            } else {
                if program[10] == 4 {
                    reg_b ^= reg_c;
                } else if program[10] == 1 {
                    reg_b ^= program[11] as u32;
                } else if program[10] == 0 {
                    reg_a >>= 3;
                }

                *OUTPUT.get_unchecked_mut(out_vals * 2) = (reg_b & 0b111) as u8 + b'0';
                out_vals += 1;
            }

            // always have 3,0
            if reg_a == 0 {
                break;
            }
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
            loop {
                reg_b = reg_a % 8;

                reg_b ^= program[3];

                reg_c = reg_a >> reg_b;

                if program[6] == 1 {
                    reg_b ^= program[7];
                } else if program[6] == 4 {
                    reg_b ^= reg_c;
                } else if program[6] == 0 {
                    reg_a >>= 3;
                }

                if program[8] == 4 {
                    reg_b ^= reg_c;
                } else if program[8] == 0 {
                    reg_a >>= 3;
                } else if program[8] == 1 {
                    reg_b ^= program[9];
                }

                if program[10] == 5 {
                    out.push(reg_b & 0b111);
                    reg_a >>= 3;
                } else {
                    if program[10] == 4 {
                        reg_b ^= reg_c;
                    } else if program[10] == 1 {
                        reg_b ^= program[11];
                    } else if program[10] == 0 {
                        reg_a >>= 3;
                    }

                    out.push(reg_b & 0b111);
                }

                // always have 3,0
                if reg_a == 0 {
                    break;
                }
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
