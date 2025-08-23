use std::collections::{HashMap, VecDeque};

pub fn part1(input: &str) -> impl std::fmt::Display {
    // module: type (&,%, or b), targets, on/off (for flip-flp), list of inputs remembered for
    // conjunctions as 0,1

    // use 0 for low, 1 for high
    let mut modules = HashMap::new();

    input.lines().for_each(|line| {
        let (mut module, target_string) = line.split_once(" -> ").unwrap();
        let targets: Vec<&str> = target_string.split(", ").collect();
        let mod_type;
        if module.starts_with('b') {
            module = "broadcaster";
            mod_type = 'b';
        } else {
            mod_type = module.chars().next().unwrap();
            module = &module[1..];
        }

        modules.insert(module, (mod_type, targets, false, HashMap::new()));
    });

    let mut modifications = Vec::new();

    let copy = modules.clone();
    for (name, data) in copy.iter() {
        for target in &data.1 {
            modifications.push((*target, name));
        }
    }
    for (target, name) in modifications {
        if let Some(entry) = modules.get_mut(target) {
            entry.3.insert(name, 0);
        }
    }

    let mut highs = 0;
    let mut lows = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back((0, "broadcaster", ""));
        while let Some(pulse) = queue.pop_front() {
            if pulse.0 == 0 {
                lows += 1;
            } else {
                highs += 1;
            }
            let to_send;
            if let Some(module) = modules.get_mut(pulse.1) {
                match module.0 {
                    'b' => {
                        to_send = pulse.0;
                    }
                    '&' => {
                        *module.3.get_mut(&pulse.2).unwrap() = pulse.0;
                        let mut all_high = true;
                        for val in module.3.values() {
                            all_high = all_high && *val == 1;
                        }
                        to_send = if all_high { 0 } else { 1 };
                    }
                    '%' => {
                        if pulse.0 == 0 {
                            module.2 = !module.2;
                            to_send = module.2 as usize;
                        } else {
                            continue;
                        }
                    }
                    _ => unreachable!(),
                }
                let targets = module.1.clone();
                for target in targets {
                    if target != "output" {
                        queue.push_back((to_send, target, pulse.1));
                    }
                }
            }
        }
    }
    highs * lows
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    // module: type (&,%, or b), targets, on/off (for flip-flp), list of inputs remembered for
    // conjunctions as 0,1

    // use 0 for low, 1 for high
    let mut modules = HashMap::new();

    input.lines().for_each(|line| {
        let (mut module, target_string) = line.split_once(" -> ").unwrap();
        let targets: Vec<&str> = target_string.split(", ").collect();
        let mod_type;
        if module.starts_with('b') {
            module = "broadcaster";
            mod_type = 'b';
        } else {
            mod_type = module.chars().next().unwrap();
            module = &module[1..];
        }

        modules.insert(module, (mod_type, targets, false, HashMap::new()));
    });

    let mut modifications = Vec::new();

    let copy = modules.clone();
    for (name, data) in copy.iter() {
        for target in &data.1 {
            modifications.push((*target, name));
        }
    }
    for (target, name) in modifications {
        if let Some(entry) = modules.get_mut(target) {
            entry.3.insert(name, 0);
        }
    }

    // got ls by manually looking which one outputs into rx
    let inputs: Vec<&str> = modules.get("ls").unwrap().3.keys().map(|s| **s).collect();

    let mut inputs_ls = HashMap::new();
    for ls_input in inputs.clone() {
        inputs_ls.insert(ls_input, 0);
    }

    let mut button_press = 0;

    loop {
        button_press += 1;
        // println!("{button_press}");
        let mut queue = VecDeque::new();
        queue.push_back((0, "broadcaster", ""));
        while let Some(pulse) = queue.pop_front() {
            let to_send;
            if let Some(module) = modules.get_mut(pulse.1) {
                match module.0 {
                    'b' => {
                        to_send = pulse.0;
                    }
                    '&' => {
                        *module.3.get_mut(&pulse.2).unwrap() = pulse.0;
                        let mut all_high = true;
                        for val in module.3.values() {
                            all_high = all_high && *val == 1;
                        }
                        to_send = if all_high { 0 } else { 1 };
                    }
                    '%' => {
                        if pulse.0 == 0 {
                            module.2 = !module.2;
                            to_send = module.2 as usize;
                        } else {
                            continue;
                        }
                    }
                    _ => unreachable!(),
                }
                let targets = module.1.clone();
                for target in targets {
                    if target != "output" {
                        queue.push_back((to_send, target, pulse.1));
                    }
                }
            }
        }

        for (key, val) in modules.get("ls").unwrap().3.iter() {
            if *val == 1 && *inputs_ls.get(*key).unwrap() == 0 {
                inputs_ls.insert(key, button_press);
                println!("{inputs_ls:#?}");
            }
        }

        let mut all_set = true;
        let mut product = 1;
        for val in inputs_ls.values() {
            all_set = all_set && *val != 0;
            product *= val;
        }
        if all_set {
            return product;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //     const INPUT: &str = "broadcaster -> a
    // %a -> inv, con
    // &inv -> b
    // %b -> con
    // &con -> output";

    const INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    #[test]
    fn part_1() {
        // assert_eq!(part1(INPUT).to_string(), String::from("11687500"))
        assert_eq!(part1(INPUT).to_string(), String::from("32000000"))
    }
}
