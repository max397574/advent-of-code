pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    input
        .lines()
        .map(|line| {
            let (l, wh) = line.split_once('x').unwrap();
            let (w, h) = wh.split_once('x').unwrap();
            let l = l.parse::<u32>().unwrap();
            let w = w.parse::<u32>().unwrap();
            let h = h.parse::<u32>().unwrap();
            let surface = 2 * l * w + 2 * w * h + 2 * h * l;
            surface + if l <= w { l * w.min(h) } else { w * l.min(h) }
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    input
        .lines()
        .map(|line| {
            let (l, wh) = line.split_once('x').unwrap();
            let (w, h) = wh.split_once('x').unwrap();
            let l = l.parse::<u32>().unwrap();
            let w = w.parse::<u32>().unwrap();
            let h = h.parse::<u32>().unwrap();
            w * l * h + 2 * (if l <= w { l + w.min(h) } else { w + l.min(h) })
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2x3x4\n";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("58"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("34"))
    }
}
