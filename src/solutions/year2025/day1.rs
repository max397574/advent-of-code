use std::intrinsics::{unchecked_mul, unchecked_sub};

/// L: 01001100 -> -1
/// R: 01010010 -> 1
#[inline(always)]
fn lr_hash(byte: u8) -> i32 {
    unsafe { std::intrinsics::unchecked_sub(1, (((byte & 4) >> 2) as i32) << 1) }
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    unsafe {
        let mut current: i32 = 50;
        let mut zero_count: u32 = 0;
        let input = input.as_bytes();
        let len = input.len();
        let mut input = input.as_ptr();
        let end_ptr = input.add(len);
        while input < end_ptr {
            let delta_sign = lr_hash(*input.offset(0));
            input = input.add(1);
            let mut delta: i32 = (unchecked_sub(*input.offset(0), b'0')) as i32;
            if *input.offset(2) == b'\n' {
                delta = delta.unchecked_mul(10);
                delta = delta.unchecked_add((unchecked_sub(*input.offset(1), b'0')) as i32);
                input = input.add(1);
            } else if *input.offset(3) == b'\n' {
                delta = unchecked_mul((unchecked_sub(*input.offset(1), b'0')) as i32, 10);
                delta = delta.unchecked_add((unchecked_sub(*input.offset(2), b'0')) as i32);
                input = input.add(2);
            }
            input = input.add(2);

            delta = delta.unchecked_mul(delta_sign);
            current = current.unchecked_add(delta);
            current = current.rem_euclid(100);
            if current == 0 {
                zero_count += 1;
            }
        }
        zero_count
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    unsafe {
        let mut current = 50;
        let mut zero_count = 0;
        let input = input.as_bytes();
        let len = input.len();
        let mut input = input.as_ptr();
        let end_ptr = input.add(len);
        while input < end_ptr {
            let delta_sign = lr_hash(*input.offset(0));
            input = input.add(1);
            let mut delta: i32 = (unchecked_sub(*input.offset(0), b'0')) as i32;
            if *input.offset(2) == b'\n' {
                delta = delta.unchecked_mul(10);
                delta = delta.unchecked_add((unchecked_sub(*input.offset(1), b'0')) as i32);
                input = input.add(1);
            } else if *input.offset(3) == b'\n' {
                delta = delta.unchecked_mul(100);
                delta = delta.unchecked_add(unchecked_mul(
                    (unchecked_sub(*input.offset(1), b'0')) as i32,
                    10,
                ));
                delta = delta.unchecked_add((unchecked_sub(*input.offset(2), b'0')) as i32);
                input = input.add(2);
            }
            input = input.add(2);

            delta = delta.unchecked_mul(delta_sign);

            if delta >= 0 {
                zero_count += (current + delta) / 100;
            } else {
                zero_count += ((100 - current) % 100 - delta) / 100;
            }
            current += delta;
            current = current.rem_euclid(100);
        }
        zero_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn hash_function() {
        assert_eq!(lr_hash(b'L'), -1);
        assert_eq!(lr_hash(b'R'), 1);
    }

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("3"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("6"))
    }
}
