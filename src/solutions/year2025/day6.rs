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
    let mut lines = input.lines().rev();
    let mut symbol_indices = Vec::new();
    lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, c)| match c {
            b'+' => symbol_indices.push((b'+', i)),
            b'*' => symbol_indices.push((b'*', i)),
            _ => (),
        });

    let bytes: Vec<&[u8]> = lines.rev().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let columns = symbol_indices.len();
    let line_count = bytes.len();

    let mut lengths = Vec::with_capacity(symbol_indices.len());
    for col in 0..columns - 1 {
        lengths.push(symbol_indices[col + 1].1 - symbol_indices[col].1 - 1);
    }
    lengths.push(input.find('\n').unwrap() - symbol_indices[symbol_indices.len() - 1].1);

    let mut results = Vec::with_capacity(columns);

    for col in 0..columns {
        let mut numbers = Vec::new();
        #[allow(clippy::needless_range_loop)]
        for col_idx in symbol_indices[col].1..symbol_indices[col].1 + lengths[col] {
            let mut num = 0;
            for line in 0..line_count {
                if bytes[line][col_idx] >= b'0' {
                    num = num * 10 + (bytes[line][col_idx] - b'0') as u64;
                }
            }
            numbers.push(num);
        }
        if symbol_indices[col].0 == b'+' {
            results.push(numbers.iter().sum::<u64>());
        } else {
            results.push(numbers.iter().product::<u64>());
        }
    }
    results.iter().sum::<u64>()
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
