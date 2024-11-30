use crate::utils::get_lcm;
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
    inspections: u128,
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
            let monkey = &mut monkeys[i];
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

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut monkeys = generator(input);
    for _ in 0..20 {
        round(&mut monkeys[..], |x| *x / 3);
    }
    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<u128>>();
    inspections.sort_unstable();
    inspections[monkeys.len() - 2..].iter().product::<u128>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut monkeys = generator(input);
    let lcm = get_lcm(monkeys.iter().map(|monkey| monkey.test).collect::<Vec<_>>());
    for _ in 0..10000 {
        round(&mut monkeys, |x| x % lcm);
    }
    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<_>>();
    inspections.sort_unstable();
    inspections[monkeys.len() - 2..].iter().product::<u128>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("10605"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("2713310158"))
    }
}
