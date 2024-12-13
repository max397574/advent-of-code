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
pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    unsafe {
        input.trim().split("\n\n").for_each(|block| {
            let (l1, rest) = block.split_once("\n").unwrap_unchecked();
            let (l2, l3) = rest.split_once("\n").unwrap_unchecked();
            let (_, block1) = l1.split_once(": X+").unwrap_unchecked();
            let (_, block2) = l2.split_once(": X+").unwrap_unchecked();
            let (_, block3) = l3.split_once(": X=").unwrap();
            let (a11, a21) = block1.split_once(", Y+").unwrap_unchecked();
            let (a12, a22) = block2.split_once(", Y+").unwrap_unchecked();
            let (b1, b2) = block3.split_once(", Y=").unwrap_unchecked();
            let (a11, a12, a21, a22, b1, b2): (isize, isize, isize, isize, isize, isize) = (
                a11.parse().unwrap_unchecked(),
                a12.parse().unwrap_unchecked(),
                a21.parse().unwrap_unchecked(),
                a22.parse().unwrap_unchecked(),
                b1.parse().unwrap_unchecked(),
                b2.parse().unwrap_unchecked(),
            );
            let det_a = a11 * a22 - a12 * a21;
            if det_a != 0 {
                let tmp1 = b1 * a22 - a12 * b2;
                let tmp2 = a11 * b2 - b1 * a21;
                if tmp1 % det_a == 0 && tmp2 % det_a == 0 {
                    let x1 = tmp1 / det_a;
                    let x2 = tmp2 / det_a;
                    if x1 >= 0 && x2 >= 0 {
                        sum += 3 * x1 + x2;
                    }
                }
            }
        });
    }
    sum
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    input.trim().split("\n\n").for_each(|block| {
        let (l1, rest) = block.split_once("\n").unwrap();
        let (l2, l3) = rest.split_once("\n").unwrap();
        let (_, block1) = l1.split_once(": X+").unwrap();
        let (_, block2) = l2.split_once(": X+").unwrap();
        let (_, block3) = l3.split_once(": X=").unwrap();
        let (a11, a21) = block1.split_once(", Y+").unwrap();
        let (a12, a22) = block2.split_once(", Y+").unwrap();
        let (b1, b2) = block3.split_once(", Y=").unwrap();
        let (a11, a12, a21, a22, b1, b2): (isize, isize, isize, isize, isize, isize) = (
            a11.parse().unwrap(),
            a12.parse().unwrap(),
            a21.parse().unwrap(),
            a22.parse().unwrap(),
            b1.parse().unwrap(),
            b2.parse().unwrap(),
        );
        let b1 = b1 + 10000000000000;
        let b2 = b2 + 10000000000000;
        let det_a = a11 * a22 - a12 * a21;
        if det_a != 0 {
            let tmp1 = b1 * a22 - a12 * b2;
            let tmp2 = a11 * b2 - b1 * a21;
            if tmp1 % det_a == 0 && tmp2 % det_a == 0 {
                let x1 = tmp1 / det_a;
                let x2 = tmp2 / det_a;
                if x1 >= 0 && x2 >= 0 {
                    sum += 3 * x1 + x2;
                }
            }
        }
    });
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
