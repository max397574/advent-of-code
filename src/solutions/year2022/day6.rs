pub fn part_1(_input: &str) -> impl std::fmt::Display {
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
    return count;
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
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
    return count;
}

// #[cfg(test)]
mod tests {
    use super::*;
    const _INPUT1: &str = "";
    const _INPUT2: &str = "";

    // #[test]
    fn _part1() {
        assert_eq!(part_1(_INPUT1).to_string(), String::from("0"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part_2(_INPUT2).to_string(), String::from("0"))
    }
}
