pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let mut lines = input.lines().rev();
    let symbols: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    let len = symbols.len();
    let mut results: Vec<u64> = vec![0; len];
    let numbers = lines.next().unwrap().split_whitespace();
    for (i, number) in numbers.enumerate() {
        results[i] = number.parse::<u64>().unwrap();
    }
    for line in lines {
        let numbers = line.split_whitespace();
        for (i, number) in numbers.enumerate() {
            match symbols[i] {
                "+" => {
                    results[i] += number.parse::<u64>().unwrap();
                }
                _ => {
                    results[i] *= number.parse::<u64>().unwrap();
                }
            }
        }
    }
    results.iter().sum::<u64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    unsafe {
        let mut lines = input.lines().rev();
        let mut symbol_indices = Vec::with_capacity(1000);
        lines
            .next()
            .unwrap_unchecked()
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, &c)| {
                if c > b' ' {
                    symbol_indices.push((c, i));
                }
            });

        let bytes: Vec<&[u8]> = lines.rev().map(|line| line.as_bytes()).collect::<Vec<_>>();
        let columns = symbol_indices.len();
        let line_count = bytes.len();

        let mut lengths = Vec::with_capacity(columns);
        for col in 0..columns - 1 {
            lengths.push(symbol_indices[col + 1].1 - symbol_indices[col].1 - 1);
        }
        lengths
            .push(input.find('\n').unwrap_unchecked() - symbol_indices[symbol_indices.len() - 1].1);

        const INITIAL_VALS: [u64; 2] = [1, 0];

        let mut res = 0;
        #[allow(clippy::needless_range_loop)]
        for col in 0..columns {
            let symbol = *symbol_indices.get_unchecked(col);
            let is_plus = symbol.0 == b'+';
            let mut val = *INITIAL_VALS.get_unchecked((symbol.0 & 1) as usize);
            for col_idx in symbol.1..symbol.1 + lengths[col] {
                let mut num = 0;
                for line in 0..line_count {
                    let val = *bytes.get_unchecked(line).get_unchecked(col_idx);
                    if val >= b'0' {
                        num = num * 10 + (val - b'0') as u64;
                    }
                }
                if is_plus {
                    val += num;
                } else {
                    val *= num;
                }
            }
            res += val;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  \n";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("4277556"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("3263827"))
    }
}
