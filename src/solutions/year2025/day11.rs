use std::collections::{HashMap, VecDeque};

fn toposort(adj: &HashMap<&str, Vec<&str>>) -> Vec<String> {
    let vertices: Vec<&str> = adj.keys().copied().collect();

    let mut indeg: HashMap<&str, usize> = HashMap::new();
    for &v in &vertices {
        indeg.insert(v, 0);
    }
    for neighbors in adj.values() {
        for v in neighbors {
            *indeg.entry(v).or_insert(0) += 1;
        }
    }

    let mut queue = VecDeque::new();
    for &v in &vertices {
        if indeg[&v] == 0 {
            queue.push_back(v);
        }
    }

    let mut order = Vec::new();
    while let Some(u) = queue.pop_front() {
        order.push(u.to_owned());
        if let Some(neighbors) = adj.get(&u) {
            for &v in neighbors {
                let deg = indeg.get_mut(&v).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(v);
                }
            }
        }
    }
    order
}

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    path_count(input, "you", "out")
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    path_count(input, "svr", "fft")
        * path_count(input, "fft", "dac")
        * path_count(input, "dac", "out")
        + path_count(input, "svr", "dac")
            * path_count(input, "dac", "fft")
            * path_count(input, "fft", "out")
}

fn path_count(input: &str, from: &str, to: &str) -> usize {
    let mut adj = HashMap::new();
    for line in input.lines() {
        let (vertex, neighbours) = line.split_once(": ").unwrap();
        let neighbours: Vec<_> = neighbours.split(' ').collect();
        adj.insert(vertex, neighbours);
    }

    let order = toposort(&adj);
    let rev_order: Vec<_> = order.iter().rev().cloned().collect();

    let mut dp: HashMap<&str, usize> = HashMap::new();

    for u in &rev_order {
        if u == to {
            dp.insert(to, 1);
            continue;
        }

        if let Some(neighbors) = adj.get(&u[..]) {
            let mut paths = 0;
            for &v in neighbors {
                paths += dp.get(v).copied().unwrap_or(0);
            }
            dp.insert(u, paths);
        }
    }

    dp.get(from).copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out\n";

    const INPUT2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out\n";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("5"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT2).to_string(), String::from("2"))
    }
}
