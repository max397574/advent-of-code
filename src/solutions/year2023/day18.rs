use crate::utils::shoelace;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut points = vec![(0, 0)];
    let mut steps = 0;
    let mut pos = (0, 0);
    input.lines().for_each(|line| {
        let mut bytes_iter = line.bytes();
        let dir = bytes_iter.next().unwrap();
        bytes_iter.next();
        let mut amount = bytes_iter.next().unwrap() - b'0';
        let mut ele = bytes_iter.next().unwrap();
        while ele.is_ascii_digit() {
            amount *= 10;
            amount += ele - b'0';
            ele = bytes_iter.next().unwrap();
        }
        let amount = amount as isize;
        steps += amount;
        pos = match dir {
            b'U' => (pos.0, pos.1 + amount),
            b'D' => (pos.0, pos.1 - amount),
            b'R' => (pos.0 + amount, pos.1),
            b'L' => (pos.0 - amount, pos.1),
            _ => unreachable!(),
        };
        points.push(pos);
    });
    let area = shoelace(&points) as isize;
    // pick's theorem
    area - steps / 2 + 1 + steps
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut points = vec![(0, 0)];
    let mut steps = 0;
    let mut pos = (0, 0);
    input.lines().for_each(|line| {
        let bytes: Vec<u8> = line.trim().bytes().collect();
        let len = bytes.len();
        let hex_str = std::str::from_utf8(&bytes[len - 7..len - 2]).unwrap();

        let amount = isize::from_str_radix(hex_str, 16).unwrap();
        let dir = bytes[len - 2] - b'0';
        steps += amount;
        pos = match dir {
            0 => (pos.0 + amount, pos.1),
            1 => (pos.0, pos.1 - amount),
            2 => (pos.0 - amount, pos.1),
            3 => (pos.0, pos.1 + amount),
            _ => unreachable!(),
        };
        points.push(pos);
    });
    let area = shoelace(&points) as isize;
    // pick's theorem
    area - steps / 2 + 1 + steps
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("62"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(INPUT).to_string(), String::from("952408144115"))
    }
}
