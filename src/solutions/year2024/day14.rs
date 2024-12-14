use bstr::ByteSlice;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let (mut one, mut two, mut three, mut four) = (0, 0, 0, 0);
    input.lines().for_each(|line| unsafe {
        //p=62,20 v=85,-14
        let mut line = line;
        line = line.get_unchecked("p=".len()..);
        let mut p_x = *line.get_unchecked(0) as i64 - b'0' as i64;
        line = line.get_unchecked(1..);
        if *line.get_unchecked(0) != b',' {
            p_x = p_x * 10 + *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            p_x = p_x * 10 + *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        line = line.get_unchecked(1..);
        let mut p_y = *line.get_unchecked(0) as i64 - b'0' as i64;
        line = line.get_unchecked(1..);
        if *line.get_unchecked(0) != b' ' {
            p_y = p_y * 10 + *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b' ' {
            p_y = p_y * 10 + *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        line = line.get_unchecked(" v=".len()..);
        let mut v_x;
        if *line.get_unchecked(0) == b'-' {
            v_x = -(*line.get_unchecked(1) as i64 - b'0' as i64);
            line = line.get_unchecked(2..);
        } else {
            v_x = *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            if v_x < 0 {
                v_x = v_x * 10 - (*line.get_unchecked(0) as i64 - b'0' as i64);
            } else {
                v_x = v_x * 10 + (*line.get_unchecked(0) as i64 - b'0' as i64);
            }
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            if v_x < 0 {
                v_x = v_x * 10 - (*line.get_unchecked(0) as i64 - b'0' as i64);
            } else {
                v_x = v_x * 10 + (*line.get_unchecked(0) as i64 - b'0' as i64);
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

fn calculate_mean(numbers: &[i64]) -> f64 {
    let sum: f64 = numbers.iter().map(|&num| num as f64).sum();
    sum / numbers.len() as f64
}

fn calculate_variance(numbers: &[i64], mean: f64) -> f64 {
    let sum_of_squared_diffs: f64 = numbers
        .iter()
        .map(|&value| {
            let value = value as f64;
            let diff = value - mean;
            diff * diff
        })
        .sum();

    sum_of_squared_diffs / (numbers.len() as f64)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut robots = Vec::new();
    let input = input.as_bytes();
    input.lines().for_each(|line| unsafe {
        //p=62,20 v=85,-14
        let mut line = line;
        line = line.get_unchecked("p=".len()..);
        let mut p_x = *line.get_unchecked(0) as i64 - b'0' as i64;
        line = line.get_unchecked(1..);
        if *line.get_unchecked(0) != b',' {
            p_x = p_x * 10 + *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            p_x = p_x * 10 + *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        line = line.get_unchecked(1..);
        let mut p_y = *line.get_unchecked(0) as i64 - b'0' as i64;
        line = line.get_unchecked(1..);
        if *line.get_unchecked(0) != b' ' {
            p_y = p_y * 10 + *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b' ' {
            p_y = p_y * 10 + *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        line = line.get_unchecked(" v=".len()..);
        let mut v_x;
        if *line.get_unchecked(0) == b'-' {
            v_x = -(*line.get_unchecked(1) as i64 - b'0' as i64);
            line = line.get_unchecked(2..);
        } else {
            v_x = *line.get_unchecked(0) as i64 - b'0' as i64;
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            if v_x < 0 {
                v_x = v_x * 10 - (*line.get_unchecked(0) as i64 - b'0' as i64);
            } else {
                v_x = v_x * 10 + (*line.get_unchecked(0) as i64 - b'0' as i64);
            }
            line = line.get_unchecked(1..);
        }
        if *line.get_unchecked(0) != b',' {
            if v_x < 0 {
                v_x = v_x * 10 - (*line.get_unchecked(0) as i64 - b'0' as i64);
            } else {
                v_x = v_x * 10 + (*line.get_unchecked(0) as i64 - b'0' as i64);
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
        robots.push(((p_x, v_x), (p_y, v_y)));
    });

    let mut bx = 0;
    let mut bxvar = f64::MAX;
    let mut by = 0;
    let mut byvar = f64::MAX;

    for t in 1..103 {
        robots.iter_mut().for_each(|((px, vx), (py, vy))| {
            *px = (*px + *vx).rem_euclid(101);
            *py = (*py + *vy).rem_euclid(103);
        });
        let xs = &robots.iter().map(|robo| (robo.0).0).collect::<Vec<_>>()[..];
        let ys = &robots.iter().map(|robo| (robo.1).0).collect::<Vec<_>>()[..];
        let mean_x = calculate_mean(xs);
        let variance_x = calculate_variance(xs, mean_x);
        let mean_y = calculate_mean(ys);
        let variance_y = calculate_variance(ys, mean_y);

        if variance_x < bxvar {
            bx = t;
            bxvar = variance_x;
        }
        if variance_y < byvar {
            by = t;
            byvar = variance_y;
        }
    }
    let mut x = bx;
    loop {
        if x % 103 == by {
            break;
        }
        x += 101;
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
