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

pub fn part2(_input: &str) -> impl std::fmt::Display {
    0
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
