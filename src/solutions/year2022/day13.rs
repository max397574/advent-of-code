#[derive(Debug, Eq, Clone)]
enum Value {
    Int(u64),
    List(Vec<Value>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(lhs), Int(rhs)) => lhs.eq(rhs),
            (List(lhs), List(rhs)) => lhs.eq(rhs),
            (List(lhs), Int(_)) => lhs.first().eq(&Some(other)),
            (Int(_), List(rhs)) => rhs.first().eq(&Some(self)),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Value::*;
        match (self, other) {
            (Int(lhs), Int(rhs)) => lhs.partial_cmp(rhs),
            (List(lhs), List(rhs)) => lhs.partial_cmp(rhs),
            (Int(lhs), List(rhs)) => ([Int(*lhs)].as_slice()).partial_cmp(rhs.as_slice()),
            (List(lhs), Int(rhs)) => lhs.as_slice().partial_cmp([Int(*rhs)].as_slice()),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<&str> for Value {
    fn from(data: &str) -> Self {
        if data == "[]" {
            return Self::List(vec![]);
        }

        if let Ok(data) = data.trim().parse::<u64>() {
            return Self::Int(data);
        }

        let range = 1..data.len() - 1;
        let chars = data[range].chars();

        let mut list = Vec::new();
        let mut buffer = String::new();
        let mut depth = 0;

        for character in chars {
            match character {
                '[' => {
                    depth += 1;
                    buffer.push(character);
                    continue;
                }
                ']' => {
                    depth -= 1;
                    buffer.push(character);
                    continue;
                }
                _ => (),
            }

            if depth == 0 {
                match character {
                    ',' => {
                        list.push(Self::from(buffer.as_str()));
                        buffer.clear();
                    }
                    _ => buffer.push(character),
                }
            } else {
                buffer.push(character);
            }
        }

        if !buffer.is_empty() {
            list.push(Self::from(buffer.as_str()))
        }

        Self::List(list)
    }
}

fn parse(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::trim)
        .map(Value::from)
        .collect()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut count = 0;
    for (i, values) in parse(input).chunks(2).enumerate() {
        if values[0] < values[1] {
            count += i + 1
        }
    }
    count
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut input = parse(input).to_owned();
    let divider1 = Value::from("[[2]]");
    let divider2 = Value::from("[[6]]");
    input.push(divider1.clone());
    input.push(divider2.clone());
    input.sort_unstable();
    let mut first = 0;
    let mut second = 0;
    for (i, item) in input.iter().enumerate() {
        if *item == divider1 {
            first = i + 1;
        }
        if *item == divider2 {
            second = i + 1;
        }
    }
    first * second
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("13"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("140"))
    }
}
