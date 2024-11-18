use bstr::ByteSlice;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut sum: u32 = 0;
    let mut tmp_sum: u32 = 0;
    input
        .as_bytes()
        .lines()
        .next()
        .unwrap()
        .iter()
        .for_each(|&b| match b {
            b',' => {
                sum += tmp_sum;
                tmp_sum = 0;
            }
            _ => {
                tmp_sum += b as u32;
                tmp_sum *= 17;
                tmp_sum %= 256;
            }
        });
    sum + tmp_sum
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT).to_string(), String::from("1320"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT).to_string(), String::from("0"))
    }
}
