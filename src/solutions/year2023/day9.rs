use atoi::atoi;
use bstr::ByteSlice;

// Each line is an arithmetic sequence of unknown order
// the next elements of each sequence on all the lines have to be summed up
pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let mut numbers: Vec<Vec<_>> = input
        .as_bytes()
        .lines()
        .map(|line| {
            line.split(|&b| b == b' ')
                .map(|num| atoi::<i64>(num).unwrap())
                .collect()
        })
        .collect();
    numbers
        .iter_mut()
        .map(|seq| {
            let mut len = seq.len();
            let mut all_zeroes = false;
            while !all_zeroes {
                all_zeroes = true;
                for i in 0..len - 1 {
                    seq[i] = seq[i + 1] - seq[i];
                    all_zeroes = all_zeroes && seq[i] == 0;
                }
                len -= 1;
            }
            seq[len..].iter().sum::<i64>()
        })
        .sum::<i64>()
}

// Each line is an arithmetic sequence of unknown order
// the previous elements of each sequence on all the lines have to be summed up
pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let mut numbers: Vec<Vec<_>> = input
        .as_bytes()
        .lines()
        .map(|line| {
            line.split(|&b| b == b' ')
                .map(|num| atoi::<i64>(num).unwrap())
                .collect()
        })
        .collect();
    numbers
        .iter_mut()
        .map(|seq| {
            let mut start = 0;
            let mut all_zeroes = false;
            while !all_zeroes {
                all_zeroes = true;
                // here we always insert the difference where the right of the two values was so we
                // can look at the values at the start
                // we have to reverse so we don't overwrite values we'll need later to calculate
                for i in (start + 1..seq.len()).rev() {
                    seq[i] -= seq[i - 1];
                    all_zeroes = all_zeroes && seq[i] == 0;
                }
                start += 1;
            }
            let mut val = 0;
            (0..start).rev().for_each(|i| val = seq[i] - val);
            val
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    const INPUT2: &str = INPUT1;

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT1).to_string(), String::from("114"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("2"))
    }
}
