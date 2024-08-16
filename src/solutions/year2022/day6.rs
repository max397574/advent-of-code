pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut last = vec![];
    let mut count = 0;
    // TODO: rewrite with std::slice::Windows
    for char in input.bytes() {
        if last.len() < 4 {
            last.push(char)
        } else {
            last.remove(0);
            last.push(char);
        }
        count += 1;
        if last.len() == 4 {
            let new_vec = last.clone();
            let mut new_vec = new_vec.clone();
            new_vec.sort();
            new_vec.dedup();
            if new_vec.len() == 4 {
                return count;
            }
        }
    }
    count
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut last = vec![];
    let mut count = 0;
    // TODO: rewrite with std::slice::Windows
    for char in input.bytes() {
        if last.len() < 14 {
            last.push(char)
        } else {
            last.remove(0);
            last.push(char);
        }
        count += 1;
        if last.len() == 14 {
            let new_vec = last.clone();
            let mut new_vec = new_vec.clone();
            new_vec.sort();
            new_vec.dedup();
            if new_vec.len() == 14 {
                return count;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT2: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT1).to_string(), String::from("10"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2).to_string(), String::from("26"))
    }
}
