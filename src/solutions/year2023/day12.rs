use crate::utils::parsing::ByteParsing;
use bstr::ByteSlice;
// bottom up solution from https://github.com/hb0nes/aoc_2023/blob/main/twelve_dp/src/main.rs

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<u8> for Spring {
    fn from(c: u8) -> Self {
        match c {
            b'.' => Spring::Operational,
            b'#' => Spring::Damaged,
            _ => Spring::Unknown,
        }
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    input
        .as_bytes()
        .lines()
        .map(|line| {
            let (springs, counts) = line.split_once(|ch| *ch == b' ').unwrap();
            let mut min_j = 0;
            let counts: Vec<_> = counts
                .split(|ch| *ch == b',')
                .map(|num| num.as_num::<usize>())
                .collect();

            let mut dp = vec![vec![0; counts.len()]; springs.len() + counts[counts.len() - 1] + 1];
            for i in 0..springs.len() {
                for j in 0..counts.len() {
                    let cur_char = &springs[i];
                    if j < min_j {
                        continue;
                    }
                    if *cur_char == b'#' && j == 0 {
                        min_j = 1;
                    }
                    if *cur_char == b'.' {
                        continue;
                    }
                    if j > 0 && dp[i][j - 1] == 0 {
                        continue;
                    }
                    if counts[j..].iter().sum::<usize>() + counts[j..].len() - 1
                        > springs[i..].len()
                    {
                        continue;
                    }
                    if (j == counts.len() - 1) && springs[i + counts[j]..].chars().any(|c| c == '#')
                    {
                        continue;
                    }
                    let max_idx = springs.len().min(i + counts[j]);
                    let end_reached = max_idx == springs.len();
                    let subsequent_character = springs.get(max_idx..max_idx + 1).unwrap_or(b"");
                    let group_valid = springs[i..i + counts[j]]
                        .chars()
                        .all(|x| x == '?' || x == '#')
                        && (end_reached || subsequent_character[0] != b'#');
                    if !group_valid {
                        continue;
                    }
                    let next_start_idx = (springs.len()).min(i + counts[j] + 1);
                    let next_broken_idx = match springs[next_start_idx..].find([b'#']) {
                        Some(n) => next_start_idx + n,
                        None => dp.len() - 1,
                    };
                    for k in next_start_idx..=next_broken_idx {
                        if j > 0 {
                            dp[k][j] += dp[i][j - 1];
                        } else {
                            dp[k][j] += 1;
                        }
                    }
                }
            }
            dp[dp.len() - 1][dp[dp.len() - 1].len() - 1]
        })
        .sum::<usize>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    part1(
        &input
            .lines()
            .map(|line| {
                let (springs, counts) = line.split_once(" ").unwrap();
                format!("{} {}", [springs; 5].join("?"), [counts; 5].join(","))
            })
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("21"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("525152"))
    }
}
