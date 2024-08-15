use std::collections::HashMap;

fn catch_up(mut tx: i32, mut ty: i32, hx: i32, hy: i32) -> (i32, i32) {
    if tx == hx && ty < hy {
        ty += 1;
    }
    if tx == hx && ty > hy {
        ty -= 1;
    }
    if ty == hy && tx < hx {
        tx += 1;
    }
    if ty == hy && tx > hx {
        tx -= 1;
    }
    if tx < hx && ty < hy {
        tx += 1;
        ty += 1;
    }
    if tx < hx && ty > hy {
        tx += 1;
        ty -= 1;
    }
    if tx > hx && ty < hy {
        tx -= 1;
        ty += 1;
    }
    if tx > hx && ty > hy {
        tx -= 1;
        ty -= 1;
    }
    (tx, ty)
}

pub fn part_1(_input: &str) -> impl std::fmt::Display {
    let mut positions = Vec::new();
    positions.push((0, 0));
    let mut hx: i32 = 0;
    let mut hy: i32 = 0;
    let mut tx: i32 = 0;
    let mut ty: i32 = 0;
    for line in input.lines() {
        let [direction, amount] = line.split_whitespace().collect::<Vec<_>>()[..] else {
            unreachable!()
        };
        for _ in 0..amount.parse().unwrap() {
            match direction {
                "U" => hy += 1,
                "D" => hy -= 1,
                "R" => hx += 1,
                "L" => hx -= 1,
                _ => unreachable!(),
            }
            if (ty - hy).abs().max((tx - hx).abs()) <= 1 {
                continue;
            }
            (tx, ty) = catch_up(tx, ty, hx, hy);
            if !positions.contains(&(tx, ty)) {
                positions.push((tx, ty));
            }
        }
    }
    positions.len()
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    let mut positions = Vec::new();
    positions.push((0, 0));
    let mut hx: i32;
    let mut hy: i32;
    let mut tx: i32;
    let mut ty: i32;
    let mut tail_positions = HashMap::new();
    for i in 0..10 {
        tail_positions.insert(i, (0, 0));
    }
    for line in input.lines() {
        let [direction, amount] = line.split_whitespace().collect::<Vec<_>>()[..] else {
            unreachable!()
        };
        for _ in 0..amount.parse().unwrap() {
            (hx, hy) = *tail_positions.get(&0).unwrap();
            match direction {
                "U" => hy += 1,
                "D" => hy -= 1,
                "R" => hx += 1,
                "L" => hx -= 1,
                _ => unreachable!(),
            }
            tail_positions.insert(0, (hx, hy));
            for i in 0..9 {
                (hx, hy) = *tail_positions.get(&i).unwrap();
                (tx, ty) = *tail_positions.get(&(i + 1)).unwrap();
                if !((ty - hy).abs().max((tx - hx).abs()) <= 1) {
                    let new_tail = catch_up(tx, ty, hx, hy);
                    tail_positions.insert(i + 1, new_tail);
                } else {
                    tail_positions.insert(i + 1, (tx, ty));
                }
            }
            let last_tail = *tail_positions.get(&9).unwrap();
            if !positions.contains(&last_tail) {
                positions.push(last_tail);
            }
        }
    }
    positions.len()
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
