use atoi::atoi;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    let mut id = 0;
    // let mut valid = true;
    let mut split = input.as_bytes().split(|&byte| byte == b' ');
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    loop {
        let Some(block) = split.next() else {
            if red <= 12 && green <= 13 && blue <= 14  {
                sum += id;
            }
            break;
        };

        if block[block.len() - 1] == b':' {
            if red <= 12 && green <= 13 && blue <= 14 {
                sum += id;
            }
            id += 1;
            red = 0;
            green = 0;
            blue = 0;
            continue;
        }

        if block[0] <= b'9' {
            let val = atoi::<u8>(block).unwrap();
            let color = split.next().unwrap();
            match color[0] {
                b'r' => red = red.max(val),
                b'g' => green = green.max(val),
                b'b' => blue = blue.max(val),
                _ => unreachable!(),
            }
        }
    }
    sum
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;
    let mut sum = 0;
    let mut split = input.as_bytes().split(|&byte| byte == b' ');
    loop {
        let Some(block) = split.next() else {
            sum += max_red * max_blue * max_green;
            break;
        };

        if block[block.len() - 1] == b':' {
            sum += max_red * max_blue * max_green;
            max_red = 0;
            max_green = 0;
            max_blue = 0;
            continue;
        }

        if block[0] <= b'9' {
            let val = atoi::<u32>(block).unwrap();
            let color = split.next().unwrap();
            match color[0] {
                b'r' => max_red = max_red.max(val),
                b'g' => max_green = max_green.max(val),
                b'b' => max_blue = max_blue.max(val),
                _ => unreachable!(),
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const _INPUT1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    const _INPUT2: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_1() {
        assert_eq!(part1(_INPUT1).to_string(), String::from("8"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(_INPUT2).to_string(), String::from("2286"))
    }
}
