use bstr::ByteSlice;
use std::intrinsics::unchecked_sub;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let input = input.as_bytes();
    let (mut one, mut two, mut three, mut four) = (0, 0, 0, 0);
    input.lines().for_each(|line| unsafe {
        //p=62,20 v=85,-14
        let mut line = line;
        line = line.get_unchecked("p=".len()..);
        let mut p_x = unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64);
        line = line.get_unchecked(1..);
        if *line.get_unchecked(0) != b',' {
            p_x = p_x * 10 + unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64);
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            p_x = p_x * 10 + unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64);
            line = line.get_unchecked(1..);
        }
        line = line.get_unchecked(1..);
        let mut p_y = unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64);
        line = line.get_unchecked(1..);
        if *line.get_unchecked(0) != b' ' {
            p_y = p_y * 10 + unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64);
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b' ' {
            p_y = p_y * 10 + unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64);
            line = line.get_unchecked(1..);
        }
        line = line.get_unchecked(" v=".len()..);
        let mut v_x;
        if *line.get_unchecked(0) == b'-' {
            v_x = -(unchecked_sub(*line.get_unchecked(1) as i64, b'0' as i64));
            line = line.get_unchecked(2..);
        } else {
            v_x = unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64);
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            if v_x < 0 {
                v_x = v_x * 10 - (unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64));
            } else {
                v_x = v_x * 10 + (unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64));
            }
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            if v_x < 0 {
                v_x = v_x * 10 - (unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64));
            } else {
                v_x = v_x * 10 + (unchecked_sub(*line.get_unchecked(0) as i64, b'0' as i64));
            }
            line = line.get_unchecked(1..);
        }
        line = line.get_unchecked(1..);
        let mut v_y;
        if *line.get_unchecked(0) == b'-' {
            v_y = -(*line.get_unchecked(1) as i64 - b'0' as i64);
            line = line.get_unchecked(2..);
        } else {
            v_y = *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b'\n' {
            if v_y < 0 {
                v_y = v_y * 10 - (*line.get_unchecked(0) as i64 - b'0' as i64);
            } else {
                v_y = v_y * 10 + (*line.get_unchecked(0) as i64 - b'0' as i64);
            }
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b'\n' {
            if v_y < 0 {
                v_y = v_y * 10 - (*line.get_unchecked(0) as i64 - b'0' as i64);
            } else {
                v_y = v_y * 10 + (*line.get_unchecked(0) as i64 - b'0' as i64);
            }
        }
        let x = (p_x + v_x * 100).rem_euclid(101);
        let y = (p_y + v_y * 100).rem_euclid(103);
        if x > 50 {
            if y > 51 {
                four += 1;
            } else if y < 51 {
                three += 1;
            }
        } else if x < 50 {
            if y > 51 {
                two += 1;
            } else if y < 51 {
                one += 1;
            }
        }
    });
    one * two * three * four
}

const W: i32 = 101;
const H: i32 = 101;

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let mut robots = Vec::new();
    let input = input.as_bytes();
    input.lines().for_each(|line| unsafe {
        let mut line = line;
        line = line.get_unchecked("p=".len()..);
        let mut p_x = *line.get_unchecked(0) as i32 - b'0' as i32;
        line = line.get_unchecked(1..);
        if *line.get_unchecked(0) != b',' {
            p_x = p_x * 10 + *line.get_unchecked(0) as i32 - b'0' as i32;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            p_x = p_x * 10 + *line.get_unchecked(0) as i32 - b'0' as i32;
            line = line.get_unchecked(1..);
        }
        line = line.get_unchecked(1..);
        let mut p_y = *line.get_unchecked(0) as i32 - b'0' as i32;
        line = line.get_unchecked(1..);
        if *line.get_unchecked(0) != b' ' {
            p_y = p_y * 10 + *line.get_unchecked(0) as i32 - b'0' as i32;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b' ' {
            p_y = p_y * 10 + *line.get_unchecked(0) as i32 - b'0' as i32;
            line = line.get_unchecked(1..);
        }
        line = line.get_unchecked(" v=".len()..);
        let mut v_x;
        if *line.get_unchecked(0) == b'-' {
            v_x = -(*line.get_unchecked(1) as i32 - b'0' as i32);
            line = line.get_unchecked(2..);
        } else {
            v_x = *line.get_unchecked(0) as i32 - b'0' as i32;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            if v_x < 0 {
                v_x = v_x * 10 - (*line.get_unchecked(0) as i32 - b'0' as i32);
            } else {
                v_x = v_x * 10 + (*line.get_unchecked(0) as i32 - b'0' as i32);
            }
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            if v_x < 0 {
                v_x = v_x * 10 - (*line.get_unchecked(0) as i32 - b'0' as i32);
            } else {
                v_x = v_x * 10 + (*line.get_unchecked(0) as i32 - b'0' as i32);
            }
            line = line.get_unchecked(1..);
        }
        line = line.get_unchecked(1..);
        let mut v_y;
        if *line.get_unchecked(0) == b'-' {
            v_y = -(*line.get_unchecked(1) as i32 - b'0' as i32);
            line = line.get_unchecked(2..);
        } else {
            v_y = *line.get_unchecked(0) as i32 - b'0' as i32;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b'\n' {
            if v_y < 0 {
                v_y = v_y * 10 - (*line.get_unchecked(0) as i32 - b'0' as i32);
            } else {
                v_y = v_y * 10 + (*line.get_unchecked(0) as i32 - b'0' as i32);
            }
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b'\n' {
            if v_y < 0 {
                v_y = v_y * 10 - (*line.get_unchecked(0) as i32 - b'0' as i32);
            } else {
                v_y = v_y * 10 + (*line.get_unchecked(0) as i32 - b'0' as i32);
            }
        }
        robots.push(((p_x, v_x), (p_y, v_y)));
    });

    let mut by = 0;
    let mut min_diff_y = 15000;
    let mut bx = 0;
    let mut min_diff_x = 15000;
    for t in 1..103 {
        let mut sum_y = 0;
        let mut sum_x = 0;
        (0..robots.len()).for_each(|i| {
            let val_y = ((robots[i].1).0 + (robots[i].1).1).rem_euclid(103);
            (robots[i].1).0 = val_y;
            sum_y += val_y.abs_diff(H / 2);
            let val_x = ((robots[i].0).0 + (robots[i].0).1).rem_euclid(101);
            (robots[i].0).0 = val_x;
            sum_x += val_x.abs_diff(W / 2);
        });
        if sum_y < min_diff_y {
            by = t;
            if min_diff_y - sum_y > 5000 {
                break;
            }
            min_diff_y = sum_y;
        }
        if sum_x < min_diff_x {
            bx = t;
            if min_diff_x - sum_x > 5000 {
                break;
            }
            min_diff_x = sum_x;
        }
    }

    bx + 51 * (by - bx) % 103 * 101
}

// #[cfg(test)]
mod tests {
    use super::*;
    const _INPUT: &str = "";

    // #[test]
    fn _part1() {
        assert_eq!(part1(_INPUT).to_string(), String::from("0"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(_INPUT).to_string(), String::from("0"))
    }
}
