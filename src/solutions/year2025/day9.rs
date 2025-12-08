use geo::{Coord, LineString, Polygon, Rect, prelude::*};
use std::cmp::{max, min};

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    let numbers = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<u64>().unwrap();
            let y = y.parse::<u64>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();
    let mut max = 0;
    for i in 0..numbers.len() - 1 {
        let a = numbers[i];
        for j in (i + 1)..numbers.len() {
            let b = numbers[j];
            max = max.max((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1));
        }
    }
    max
}

fn find_max_rectangle(
    input: &str,
    polygon: &Polygon<f64>,
) -> Option<(u64, (u64, u64), (u64, u64))> {
    let numbers: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<u64>().unwrap();
            let y = y.parse::<u64>().unwrap();
            (x, y)
        })
        .collect();

    let mut huge_x_diff_points = Vec::new();
    numbers.windows(2).for_each(|window| {
        let [(ax, ay), (bx, by)] = window else {
            unreachable!()
        };
        if ax.abs_diff(*bx) >= 20000 {
            huge_x_diff_points.push((ax, ay));
            huge_x_diff_points.push((bx, by));
        }
    });

    // Find the two rightmost points (big x values)
    let right_points: Vec<_> = huge_x_diff_points
        .iter()
        .filter(|&&(_, y)| y == huge_x_diff_points[1].1 || y == huge_x_diff_points[3].1)
        .cloned()
        .collect();

    let mut max_area = 0u64;
    let mut best_rect = None;

    for &(right_x, right_y) in &right_points {
        // Try rectangles with bottom-left corners left and above/below
        for &(lx, ly) in &numbers {
            if lx >= *right_x {
                continue;
            }

            // Check both directions for height: above and below right_y
            for y_dir in &[1, -1] {
                let test_ly = if *y_dir > 0 {
                    // Above: smaller y values
                    (*right_y as i64 - 1).max(ly as i64) as u64
                } else {
                    // Below: larger y values
                    (ly as i64 + 1).min(*right_y as i64) as u64
                };

                if test_ly == *right_y || test_ly == 0 {
                    continue;
                }

                let rect_f64 = Rect::new(
                    Coord {
                        x: lx as f64,
                        y: min(test_ly, *right_y) as f64,
                    },
                    Coord {
                        x: *right_x as f64,
                        y: max(test_ly, *right_y) as f64,
                    },
                );

                // Check if entire rectangle is inside polygon
                if polygon.contains(&rect_f64) {
                    let area = (right_x - lx + 1) * (right_y.abs_diff(test_ly) + 1);
                    if area > max_area {
                        max_area = area;
                        best_rect = Some((area, (lx, test_ly), (*right_x, *right_y)));
                    }
                }
            }
        }
    }

    best_rect
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let numbers = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<f64>().unwrap();
            let y = y.parse::<f64>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let polygon = Polygon::new(LineString::from(numbers), vec![]);

    if let Some((area, bl, tr)) = find_max_rectangle(input, &polygon) {
        // println!("Max rectangle area: {}", area);
        // println!("Bottom-left: {:?}, Top-right: {:?}", bl, tr);
        return area;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3\n";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("50"))
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), String::from("24"))
    }
}
