use bstr::ByteSlice;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
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

#[derive(Clone, Copy)]
struct Lens {
    label: u32,
    focal_length: u8,
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let mut boxes: [Vec<Lens>; 256] = [const { Vec::new() }; 256];
    input.split(',').for_each(|instr| {
        let mut hash = 0;
        let mut label = 0;
        let mut label_factor = 1;
        let mut i = 0;
        while i < instr.len() {
            let b = *instr.as_bytes().get(i).unwrap();
            match b {
                b'-' => {
                    let idx = boxes[hash].iter().position(|lens| lens.label == label);
                    if let Some(index) = idx {
                        boxes[hash].remove(index);
                    }
                }
                b'=' => {
                    let idx = boxes[hash].iter().position(|lens| lens.label == label);
                    if let Some(index) = idx {
                        boxes[hash][index].focal_length =
                            *instr.as_bytes().get(i + 1).unwrap() - b'0';
                    } else {
                        boxes[hash].push(Lens {
                            label,
                            focal_length: *instr.as_bytes().get(i + 1).unwrap() - b'0',
                        });
                    }
                    i += 1;
                }
                _ => {
                    label += label_factor * ((b - b'a') as u32);
                    label_factor <<= 8;
                    hash += b as usize;
                    hash *= 17;
                    hash %= 256;
                }
            }
            i += 1;
        }
    });
    let mut res = 0;
    for (i, lens_box) in boxes.iter().enumerate() {
        for (idx, lens) in lens_box.iter().enumerate() {
            res += (i + 1) * (idx + 1) * lens.focal_length as usize;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("1320"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("145"))
    }
}
