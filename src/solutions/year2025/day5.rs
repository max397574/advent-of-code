pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();
    let mut ranges = Vec::new();
    for range in fresh_ranges.lines() {
        let (start, end) = range.split_once('-').unwrap();
        ranges.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
    }
    let mut fresh_ones = 0;
    'outer: for ingredient in ingredients.lines() {
        let ingredient = ingredient.parse::<u64>().unwrap();
        for (start, end) in &ranges {
            if (start..=end).contains(&&ingredient) {
                fresh_ones += 1;
                continue 'outer;
            }
        }
    }
    fresh_ones
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let mut max = 0;
    let (fresh_ranges, _) = input.split_once("\n\n").unwrap();
    let mut ranges = Vec::new();
    for range in fresh_ranges.lines() {
        let (start, end) = range.split_once('-').unwrap();
        ranges.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
        max = max.max(end.parse::<u64>().unwrap());
    }
    let mut fresh_ones = 0;
    ranges.sort_unstable();

    let mut prev_end = 0;

    for (start, end) in ranges {
        let start = if prev_end > start { prev_end } else { start };
        prev_end = if prev_end > end + 1 {
            prev_end
        } else {
            end + 1
        };
        fresh_ones += prev_end - start;
    }
    fresh_ones
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32\n";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("3"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("14"))
    }
}
