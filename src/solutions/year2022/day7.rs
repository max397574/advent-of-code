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

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    generator(input)
        .values()
        .filter(|x| x <= &&100000)
        .sum::<usize>()
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let input = generator(input);
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

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("95437"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("24933642"))
    }
}
