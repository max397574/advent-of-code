// https://github.com/orlp/aoc2022/blob/17c1c84f89715afd0e3b8a229da44567f14b3397/src/bin/day08_v2.rs
// scan each row in one pass in the same order you are 'looking' and keep a stack of unblocked trees
// in reverse sorted order of height, then when you meet a higher/equal tree you can instantly find
// out which trees are blocked, and pop them from the stack. This way the stack stays in reverse
// sorted order automatically, without having to sort.

use crate::utils::grid;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let cells = grid(input, |byte| byte.to_digit(10).unwrap() as i16);
    let len = cells.len();
    let row_len = cells[0].len();
    let mut visible = 0;
    let mut maximums = Vec::new();
    let mut seen: Vec<Vec<i16>> = Vec::new();
    for i in 0..len {
        seen.push(Vec::new());
        let len = row_len;
        for _ in 0..len {
            seen[i].push(0);
        }
        maximums.push(-1);
    }
    for i in 0..len {
        for j in 0..row_len {
            if cells[i][j] > maximums[i] {
                maximums[i] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }
    maximums.clear();
    for _ in 0..len {
        maximums.push(-1);
    }
    for i in 0..len {
        for x in 0..row_len {
            let j = row_len - x - 1;
            if cells[i][j] > maximums[i] {
                maximums[i] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    }

    maximums.clear();
    for _ in 0..row_len {
        maximums.push(-1);
    }
    (0..row_len).for_each(|j| {
        for i in 0..len {
            if cells[i][j] > maximums[j] {
                maximums[j] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    });
    maximums.clear();
    for _ in 0..row_len {
        maximums.push(-1);
    }
    (0..row_len).for_each(|j| {
        for x in 0..len {
            let i = len - x - 1;
            if cells[i][j] > maximums[j] {
                maximums[j] = cells[i][j];
                if seen[i][j] == 0 {
                    seen[i][j] = 1;
                    visible += 1;
                }
            }
        }
    });
    visible
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let cells = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|byte| byte.to_digit(10).unwrap() as i16)
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>();
    let len = cells.len();
    let row_len = cells[0].len();
    let mut maximum = 0;
    for i in 1..len - 1 {
        for j in 1..row_len - 1 {
            let height = cells[i][j];
            // above
            let mut i2 = i;
            let mut j2 = j;
            let mut right = 0;
            let mut left = 0;
            let mut above = 0;
            let mut below = 0;
            // above
            while i2 > 0 {
                i2 -= 1;
                above += 1;
                if cells[i2][j] >= height {
                    i2 = 0;
                }
            }
            i2 = i;
            // below
            while i2 < len - 1 {
                i2 += 1;
                below += 1;
                if cells[i2][j] >= height {
                    i2 = len;
                }
            }
            // left
            while j2 > 0 {
                j2 -= 1;
                left += 1;
                if cells[i][j2] >= height {
                    j2 = 0;
                }
            }
            j2 = j;
            // right
            while j2 < row_len - 1 {
                j2 += 1;
                right += 1;
                if cells[i][j2] >= height {
                    j2 = row_len;
                }
            }
            if (left * right * above * below) > maximum {
                maximum = left * right * above * below;
            }
        }
    }

    maximum
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("21"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("8"))
    }
}
