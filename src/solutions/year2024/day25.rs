pub fn part1(input: &str) -> usize {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for block in input.split("\n\n") {
        let mut key_or_lock = vec![0, 0, 0, 0, 0];
        for line in block.lines() {
            for (idx, char) in line.chars().enumerate() {
                if char == '#' {
                    key_or_lock[idx] += 1;
                }
            }
        }
        if block.starts_with('#') {
            locks.push(key_or_lock);
        } else {
            keys.push(key_or_lock);
        }
    }
    let mut count = 0;
    for key in keys {
        for lock in locks.iter() {
            if key.iter().zip(lock).all(|(k, l)| k + l <= 7) {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("3"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(INPUT).to_string(), String::from("0"))
    }
}
