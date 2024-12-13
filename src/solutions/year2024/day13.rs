// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// can be converted to
// 94a + 22b = 8400
// 34a + 67b = 5400
//
// gives
// |94,34| * |a| = 8400
// |22,67] * |b| = 5400

// |a11,a12| * |x1| = b1
// |a21,a22] * |x2| = b2

// using cramers rule:
// x1 = det(b1,a12;b2,a22)/det(a11,a12;a21,a22)
// x2 = det(a11,b1;a21,b2)/det(a11,a12;a21,a22)

// result: 3x1 + x2

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut input = input.as_bytes();
    let mut sum = 0;
    unsafe {
        loop {
            input = input.get_unchecked("Button A: X+".len()..);
            let a11 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 10
                + (*input.get_unchecked(1) as i64 - b'0' as i64);
            input = input.get_unchecked("00, Y+".len()..);
            let a21 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 10
                + (*input.get_unchecked(1) as i64 - b'0' as i64);
            input = input.get_unchecked("00\nButton B: X+".len()..);
            let a12 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 10
                + (*input.get_unchecked(1) as i64 - b'0' as i64);
            input = input.get_unchecked("00, Y+".len()..);
            let a22 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 10
                + (input.get_unchecked(1) - b'0') as i64;
            input = input.get_unchecked("00\nPrize: X=".len()..);
            let mut b1 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 100
                + (*input.get_unchecked(1) as i64 - b'0' as i64) * 10
                + (*input.get_unchecked(2) as i64 - b'0' as i64);
            input = input.get_unchecked(3..);
            while *input.get_unchecked(0) != b',' {
                b1 = 10 * b1 + (*input.get_unchecked(0) as i64 - b'0' as i64);
                input = input.get_unchecked(1..);
            }
            input = input.get_unchecked(4..);
            let mut b2 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 100
                + (*input.get_unchecked(1) as i64 - b'0' as i64) * 10
                + (*input.get_unchecked(2) as i64 - b'0' as i64);
            input = input.get_unchecked(3..);
            while !input.is_empty() && *input.get_unchecked(0) != b'\n' {
                b2 = 10 * b2 + (*input.get_unchecked(0) as i64 - b'0' as i64);
                input = input.get_unchecked(1..);
            }
            let det_a = a11 * a22 - a12 * a21;
            if det_a != 0 {
                let tmp1 = b1 * a22 - a12 * b2;
                let tmp2 = a11 * b2 - b1 * a21;
                //TODO: unchecked modulo or sth?
                if tmp1 % det_a == 0 && tmp2 % det_a == 0 {
                    // TODO: use unchecked div
                    let x1 = tmp1 / det_a;
                    let x2 = tmp2 / det_a;
                    if x1 >= 0 && x2 >= 0 {
                        sum += 3 * x1 + x2;
                    }
                }
            }
            if input.len() <= 1 {
                break;
            }
            input = input.get_unchecked(2..);
        }
    }
    sum
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut input = input.as_bytes();
    let mut sum = 0;
    unsafe {
        loop {
            input = input.get_unchecked("Button A: X+".len()..);
            let a11 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 10
                + (*input.get_unchecked(1) as i64 - b'0' as i64);
            input = input.get_unchecked("00, Y+".len()..);
            let a21 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 10
                + (*input.get_unchecked(1) as i64 - b'0' as i64);
            input = input.get_unchecked("00\nButton B: X+".len()..);
            let a12 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 10
                + (*input.get_unchecked(1) as i64 - b'0' as i64);
            input = input.get_unchecked("00, Y+".len()..);
            let a22 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 10
                + (input.get_unchecked(1) - b'0') as i64;
            input = input.get_unchecked("00\nPrize: X=".len()..);
            let mut b1 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 100
                + (*input.get_unchecked(1) as i64 - b'0' as i64) * 10
                + (*input.get_unchecked(2) as i64 - b'0' as i64);
            input = input.get_unchecked(3..);
            while *input.get_unchecked(0) != b',' {
                b1 = 10 * b1 + (*input.get_unchecked(0) as i64 - b'0' as i64);
                input = input.get_unchecked(1..);
            }
            input = input.get_unchecked(4..);
            let mut b2 = (*input.get_unchecked(0) as i64 - b'0' as i64) * 100
                + (*input.get_unchecked(1) as i64 - b'0' as i64) * 10
                + (*input.get_unchecked(2) as i64 - b'0' as i64);
            input = input.get_unchecked(3..);
            while !input.is_empty() && *input.get_unchecked(0) != b'\n' {
                b2 = 10 * b2 + (*input.get_unchecked(0) as i64 - b'0' as i64);
                input = input.get_unchecked(1..);
            }
            let b1 = b1 + 10000000000000;
            let b2 = b2 + 10000000000000;

            let det_a = a11 * a22 - a12 * a21;
            if det_a != 0 {
                let tmp1 = b1 * a22 - a12 * b2;
                let tmp2 = a11 * b2 - b1 * a21;
                //TODO: unchecked modulo or sth?
                if tmp1 % det_a == 0 && tmp2 % det_a == 0 {
                    // TODO: use unchecked div
                    let x1 = tmp1 / det_a;
                    let x2 = tmp2 / det_a;
                    if x1 >= 0 && x2 >= 0 {
                        sum += 3 * x1 + x2;
                    }
                }
            }
            if input.len() <= 1 {
                break;
            }
            input = input.get_unchecked(2..);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("480"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(INPUT).to_string(), String::from("0"))
    }
}
