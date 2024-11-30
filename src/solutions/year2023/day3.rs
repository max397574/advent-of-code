use atoi::atoi;
use bstr::ByteSlice;

fn get_grid(input: &[u8]) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    let lines = input.lines();
    let length = input.find_byte(b'\n').unwrap();
    grid.push(vec![b'.'; length + 2]);
    for line in lines {
        let mut new_line = Vec::new();
        new_line.push(b'.');
        new_line.extend(line);
        new_line.push(b'.');
        grid.push(new_line);
    }
    grid.push(vec![b'.'; length + 2]);
    grid
}

fn is_symbol(char: u8) -> bool {
    char != b'.' && !char.is_ascii_digit()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    let grid = get_grid(input.as_bytes());
    //for line in grid.iter() {
    //    println!("{:?}", line.to_str().unwrap());
    //}
    for (idx, line) in grid.iter().enumerate().skip(1).rev().skip(1).rev() {
        let mut skip: usize = 0;
        while skip < line.len() {
            if !line[skip].is_ascii_digit() {
                skip += 1;
                continue;
            }
            for (col, byte) in line.iter().enumerate().skip(skip) {
                if !byte.is_ascii_digit() {
                    let num = &line[skip..col];
                    //println!("{:?}", num.to_str().unwrap());
                    if is_symbol(line[skip - 1])
                        || is_symbol(line[col])
                        || grid[idx - 1]
                            .iter()
                            .skip(skip - 1)
                            .rev()
                            .skip(line.len() - col - 1)
                            .any(|b| is_symbol(*b))
                        || grid[idx + 1]
                            .iter()
                            .skip(skip - 1)
                            .rev()
                            .skip(line.len() - col - 1)
                            .any(|b| is_symbol(*b))
                    {
                        // println!("{:?}", num.to_str().unwrap());
                        sum += num.to_str().unwrap().parse::<u32>().unwrap();
                    }
                    skip += num.len();
                    break;
                }
            }
        }
    }
    sum
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    let grid = get_grid(input.as_bytes());
    for (idx, line) in grid.iter().enumerate().skip(1).rev().skip(1).rev() {
        let mut skip = 0;
        'cur_asteriks: while let Some(i) = line[skip..].find_byte(b'*') {
            skip += i + 1;
            let mut number_count = 0;
            let mut found_numbers = [(0, 0); 6];
            let mut result = 1;
            for col in 0..3 {
                for row in 0..3 {
                    let y = idx + row - 1;
                    let x = skip + col - 2;
                    if grid[y][x].is_ascii_digit() {
                        let line = &grid[y];
                        let mut start_col = x;
                        let mut end_col = x;

                        while line[start_col - 1].is_ascii_digit() {
                            start_col -= 1;
                        }

                        if found_numbers.contains(&(start_col, row)) {
                            continue;
                        }

                        found_numbers[number_count] = (start_col, row);

                        number_count += 1;
                        if number_count > 2 {
                            continue 'cur_asteriks;
                        }

                        while line[end_col + 1].is_ascii_digit() {
                            end_col += 1;
                        }

                        result *= atoi::<u32>(&line[start_col..=end_col]).unwrap()
                    }
                }
            }
            if number_count == 2 {
                sum += result;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const _INPUT1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    const _INPUT2: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1() {
        assert_eq!(part1(_INPUT1).to_string(), String::from("4361"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(_INPUT2).to_string(), String::from("467835"))
    }
}
