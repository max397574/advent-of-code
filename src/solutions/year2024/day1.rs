use bstr::ByteSlice;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let line_length = unsafe { input.find_byte(b'\n').unwrap_unchecked() + 1 };
    let lines = input.len() / line_length;
    let mut col1: Vec<u64> = vec![0; lines];
    let mut col2: Vec<u64> = vec![0; lines];
    let idx1 = unsafe { input.iter().position(|b| *b == b' ').unwrap_unchecked() };
    let idx2 = idx1 + 3;
    let idx3 = idx1 + idx2;
    input.chunks(line_length).enumerate().for_each(|(i, line)| {
        col1[i] = swar(u64::from_be_bytes(line[0..idx2].try_into().unwrap()) >> 24);
        col2[i] = swar(u64::from_be_bytes(line[idx1..idx3].try_into().unwrap()) & 0xffffffffff);
    });
    col1.sort_unstable();
    col2.sort_unstable();
    col1.iter()
        .zip(col2)
        .map(|(val1, val2)| val1.abs_diff(val2))
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let line_length = unsafe { input.find_byte(b'\n').unwrap_unchecked() + 1 };
    let lines = input.len() / line_length;
    let mut col1: Vec<usize> = vec![0; lines];
    let mut counts: [u16; 100000] = [0; 100000];
    let idx1 = unsafe { input.find_byte(b' ').unwrap_unchecked() };
    let idx2 = idx1 + 3;
    let idx3 = idx1 + idx2;
    input.lines().enumerate().for_each(|(i, line)| {
        col1[i] = swar(u64::from_be_bytes(line[0..idx2].try_into().unwrap()) >> 24) as usize;
        counts[swar(u64::from_be_bytes(line[idx1..idx3].try_into().unwrap()) & 0xffffffffff)
            as usize] += 1;
    });

    col1.iter()
        .map(|num| num * counts[*num] as usize)
        .sum::<usize>()
}

#[inline]
fn swar(chunk: u64) -> u64 {
    let lower = chunk & 0x000f000f000f000f;
    let upper = (chunk & 0x0f000f000f000f00) >> 8;
    let chunk = 10 * upper + lower;

    let lower = chunk & 0x000000ff000000ff;
    let upper = (chunk & 0x00ff000000ff0000) >> 16;
    let chunk = 100 * upper + lower;

    let lower = chunk & 0x000000000000ffff;
    let upper = (chunk & 0x0000ffff00000000) >> 32;
    10000 * upper + lower
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("11"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("31"))
    }
}
