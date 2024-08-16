struct Game {
    numbers: Vec<u32>,
    boards: Vec<Vec<Vec<u32>>>,
}

impl Game {
    fn make_move(&mut self, num: u32) {
        self.boards.iter_mut().for_each(|board| {
            (0..5).for_each(|row| {
                (0..5).for_each(|col| {
                    if board[row][col] == num {
                        board[row][col] = 0;
                    }
                })
            })
        });
    }

    fn get_bingos(&self) -> Vec<bool> {
        let mut boards = vec![false; self.boards.len()];
        self.boards.iter().enumerate().for_each(|(idx, board)| {
            for row in board.iter().take(5) {
                if row.iter().sum::<u32>() == 0 {
                    boards[idx] = true;
                    break;
                }
            }
            for col in 0..5 {
                if (0..5).map(|row| board[row][col]).sum::<u32>() == 0 {
                    boards[idx] = true;
                    break;
                }
            }
        });
        boards
    }

    fn uncovered_sum(&self, board: usize) -> u32 {
        let mut sum = 0;
        (0..5).for_each(|row| {
            (0..5).for_each(|col| {
                sum += self.boards[board][row][col];
            })
        });
        sum
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let (numbers, boards) = input.split_once("\n\n").unwrap();
    let numbers = numbers
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    let boards = boards
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    let mut game = Game { boards, numbers };
    for num in game.numbers.clone() {
        game.make_move(num);
        let bingos = game.get_bingos();
        if bingos.iter().any(|bingo| *bingo) {
            let idx = bingos.iter().position(|&bingo| bingo).unwrap();
            return game.uncovered_sum(idx) * num;
        }
    }
    0
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let (numbers, boards) = input.split_once("\n\n").unwrap();
    let numbers = numbers
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    let boards = boards
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    let mut game = Game { boards, numbers };
    let mut found_bingos = vec![false; game.boards.len()];
    for num in game.numbers.clone() {
        game.make_move(num);
        let bingos = game.get_bingos();
        if bingos.iter().all(|bingo| *bingo) {
            let idx = found_bingos.iter().position(|&bingo| !bingo).unwrap();
            return game.uncovered_sum(idx) * num;
        }
        found_bingos = bingos.clone();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT).to_string(), String::from("4512"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT).to_string(), String::from("1924"))
    }
}
