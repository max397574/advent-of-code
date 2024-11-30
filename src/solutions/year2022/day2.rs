pub fn part1(input: &str) -> impl std::fmt::Display {
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

pub fn part2(input: &str) -> impl std::fmt::Display {
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

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("15"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("12"))
    }
}
