pub fn part_1(_input: &str) -> impl std::fmt::Display {
    let lines = input.lines();
    let mut score = 0;
    for line in lines {
        let (opponent, player): (i32, i32) = match line {
            "A X" => (1, 1),
            "A Y" => (1, 2),
            "A Z" => (1, 3),
            "B X" => (2, 1),
            "B Y" => (2, 2),
            "B Z" => (2, 3),
            "C X" => (3, 1),
            "C Y" => (3, 2),
            "C Z" => (3, 3),
            _ => unreachable!(),
        };
        score += player;
        if opponent == player {
            score += 3;
        } else if (player - opponent) == 1 || (player == 1 && opponent == 3) {
            score += 6;
        }
    }
    score
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    let lines = input.lines();
    let mut score = 0;
    for line in lines {
        let (opponent, result) = match line {
            "A X" => (1, 1),
            "A Y" => (1, 2),
            "A Z" => (1, 3),
            "B X" => (2, 1),
            "B Y" => (2, 2),
            "B Z" => (2, 3),
            "C X" => (3, 1),
            "C Y" => (3, 2),
            "C Z" => (3, 3),
            _ => unreachable!(),
        };
        score += (result - 1) * 3;
        // you have to loose
        if result == 1 {
            if opponent == 1 {
                score += 3;
            } else {
                score += opponent - 1;
            }
        // you have to draw
        } else if result == 2 {
            // take same as opponent
            score += opponent;
        // you have to win
        } else if opponent == 3 {
            score += 1;
        } else {
            score += opponent + 1;
        }
    }
    score
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
