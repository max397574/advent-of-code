use std::collections::HashMap;

fn generator(input: &str) -> HashMap<String, usize> {
    let mut cwd: Vec<String> = Vec::new();
    let mut dir_sizes = HashMap::new();
    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => {
                cwd = vec!["".to_string()];
            }
            ["$", "cd", ".."] => {
                cwd.pop();
            }
            ["$", "cd", x] => {
                cwd.push(x.to_string());
            }
            ["$", "ls"] => {}
            ["dir", _] => {}
            [size, _] => {
                let mut dir_name = String::from("/");
                for dir in cwd.iter() {
                    dir_name.push('/');
                    dir_name.push_str(dir);
                    let entry = dir_sizes.entry(dir_name.clone()).or_insert(0);
                    *entry += size.parse::<usize>().unwrap();
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
    dir_sizes
}

pub fn part_1(_input: &str) -> impl std::fmt::Display {
    input.values().filter(|x| x <= &&100000).sum()
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    let needed = 40000000;
    let cur = input.get("//").unwrap();
    let mut min = 50000000;
    for &size in input.values() {
        if size >= cur - needed && size < min {
            min = size;
        }
    }
    min
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
