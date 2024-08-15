use aoc::get_lcm;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

impl Operation {
    fn apply(&self, old: usize) -> usize {
        match self {
            Operation::Add(x) => x + old,
            Operation::Mul(x) => x * old,
            Operation::Square => old * old,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: usize,
    success: usize,
    failure: usize,
    inspections: usize,
}

fn generator(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    for monkey in input.split("\n\n") {
        let mut op = Operation::Square;
        let lines = monkey
            .lines()
            .filter_map(|l| l.split(':').last())
            .collect::<Vec<&str>>();
        let items_vec: VecDeque<usize> = lines[1].trim().split(", ").flat_map(str::parse).collect();
        let operation_parts: Vec<&str> = lines[2]
            .split(" = ")
            .last()
            .unwrap()
            .split_whitespace()
            .collect();
        if operation_parts[2] == "old" {
            op = Operation::Square;
        } else if operation_parts[1] == "*" {
            op = Operation::Mul(operation_parts[2].parse().unwrap());
        } else if operation_parts[1] == "+" {
            op = Operation::Add(operation_parts[2].parse().unwrap());
        }
        let test = lines[3]
            .split(": ")
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .expect("Invalid test")
            .parse()
            .unwrap();
        let success = lines[4]
            .split(": ")
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let failure = lines[5]
            .split(": ")
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        monkeys.push(Monkey {
            operation: op,
            test,
            success,
            failure,
            items: items_vec,
            inspections: 0,
        })
    }
    monkeys
}

fn round(monkeys: &mut [Monkey], callback: impl Fn(&usize) -> usize) {
    for i in 0..monkeys.len() {
        while let Some(item) = monkeys[i].items.pop_front() {
            let mut monkey = &mut monkeys[i];
            let new = callback(&monkey.operation.apply(item));
            monkey.inspections += 1;
            if new % monkeys[i].test == 0 {
                monkeys[monkeys[i].success].items.push_back(new)
            } else {
                monkeys[monkeys[i].failure].items.push_back(new)
            }
        }
    }
}

pub fn part_1(_input: &str) -> impl std::fmt::Display {
    for _ in 0..20 {
        round(monkeys, |x| *x / 3);
    }
    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<_>>();
    inspections.sort_unstable();
    inspections[monkeys.len() - 2..].iter().product()
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    let lcm = get_lcm(monkeys.iter().map(|monkey| monkey.test).collect::<Vec<_>>());
    for _ in 0..10000 {
        round(monkeys, |x| x % lcm);
    }
    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<_>>();
    inspections.sort_unstable();
    inspections[monkeys.len() - 2..].iter().product()
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
