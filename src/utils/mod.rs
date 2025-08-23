pub mod graphs;
pub mod grid;
pub mod parsing;

pub fn shoelace(vertices: &[(isize, isize)]) -> f64 {
    let mut sum = 0;
    let len = vertices.len();
    for i in 0..len - 1 {
        sum += vertices[i].0 * vertices[i + 1].1;
        sum -= vertices[i].1 * vertices[i + 1].0;
    }

    sum += vertices[len - 1].0 * vertices[0].1;
    sum -= vertices[len - 1].1 * vertices[0].0;

    0.5 * (sum as f64).abs()
}

pub fn grid<T>(text_block: &str, callback: impl Fn(char) -> T) -> Vec<Vec<T>> {
    text_block
        .lines()
        .map(|line| line.chars().map(&callback).collect())
        .collect()
}

pub fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    let mut c;
    while b != 0 {
        c = a;
        a = b;
        b = c % b;
    }
    a
}

pub fn single_lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    pub fn get_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

pub fn get_lcm(numbers: Vec<usize>) -> usize {
    let mut tmp_lcm = single_lcm(*numbers.first().unwrap(), *numbers.get(1).unwrap());
    for num in numbers.iter().skip(2) {
        tmp_lcm = single_lcm(tmp_lcm, *num);
    }
    tmp_lcm
}
