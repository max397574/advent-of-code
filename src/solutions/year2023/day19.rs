use crate::utils::parsing::ByteParsing;
use std::collections::HashMap;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let rules: HashMap<&str, Vec<(char, char, u32, &str)>> = rules
        .lines()
        .map(|line| {
            let (name, sub_rules) = line[..line.len() - 1].split_once('{').unwrap();
            let sub_rules = sub_rules.split(',');
            (
                name,
                sub_rules
                    .map(|rule| {
                        if !rule.contains(':') {
                            return ('x', '<', u32::MAX, rule);
                        }
                        let (val, target) = rule[2..].split_once(':').unwrap();
                        (
                            rule.as_bytes()[0] as char,
                            rule.as_bytes()[1] as char,
                            val.as_bytes().as_num(),
                            target,
                        )
                    })
                    .collect(),
            )
        })
        .collect();
    let parts: Vec<(u32, u32, u32, u32)> = parts
        .lines()
        .map(|line| {
            let line_parts: Vec<&[u8]> = line
                .as_bytes()
                .split(|&val| val == b'=' || val == b',' || val == b'}')
                .collect();
            (
                line_parts[1].as_num(),
                line_parts[3].as_num(),
                line_parts[5].as_num(),
                line_parts[7].as_num(),
            )
        })
        .collect();
    parts
        .iter()
        .filter(|part| {
            let mut rule = &rules["in"];
            'outer: loop {
                for (which, comparator, val, next) in rule {
                    let part_val = match which {
                        'x' => part.0,
                        'm' => part.1,
                        'a' => part.2,
                        's' => part.3,
                        _ => unreachable!(),
                    };

                    if if *comparator == '<' {
                        part_val < *val
                    } else {
                        part_val > *val
                    } {
                        match *next {
                            "A" => {
                                return true;
                            }
                            "R" => {
                                return false;
                            }
                            _ => {
                                rule = &rules[next];
                                continue 'outer;
                            }
                        }
                    }
                }
            }
        })
        .map(|part| part.0 + part.1 + part.2 + part.3)
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let (rules, _) = input.split_once("\n\n").unwrap();
    let rules: HashMap<&str, Vec<(char, char, u64, &str)>> = rules
        .lines()
        .map(|line| {
            let (name, sub_rules) = line[..line.len() - 1].split_once('{').unwrap();
            let sub_rules = sub_rules.split(',');
            (
                name,
                sub_rules
                    .map(|rule| {
                        if !rule.contains(':') {
                            return ('x', '<', u64::MAX - 1, rule);
                        }
                        let (val, target) = rule[2..].split_once(':').unwrap();
                        (
                            rule.as_bytes()[0] as char,
                            rule.as_bytes()[1] as char,
                            val.as_bytes().as_num(),
                            target,
                        )
                    })
                    .collect(),
            )
        })
        .collect();

    let mut count = 0;
    let mut todo = vec![("in", [(1, 4001); 4])];
    while let Some((rule, mut boundaries)) = todo.pop() {
        for &(c, comparator, val, target) in &rules[rule] {
            let (lower, upper) = boundaries[match c {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => unreachable!(),
            }];
            let val = Ord::clamp(val + if comparator == '>' { 1 } else { 0 }, lower, upper);
            let (valid, invalid) = match comparator {
                '>' => ((val, upper), (lower, val)),
                '<' => ((lower, val), (val, upper)),
                _ => unreachable!(),
            };

            if valid.0 < valid.1 {
                boundaries[match c {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => unreachable!(),
                }] = valid;

                match target {
                    "R" => {}
                    "A" => {
                        count += boundaries
                            .iter()
                            .map(|(low, high)| high - low)
                            .product::<u64>()
                    }
                    _ => {
                        todo.push((target, boundaries));
                    }
                }
            }

            if invalid.0 >= invalid.1 {
                break;
            }

            boundaries[match c {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => unreachable!(),
            }] = invalid;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("19114"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("167409079868000"))
    }
}
