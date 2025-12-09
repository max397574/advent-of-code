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

use std::cmp::{max, min};

#[derive(Debug, Clone)]
struct Polygon {
    points: Vec<(f64, f64)>,
}

impl Polygon {
    pub fn contains(&self, rect: &Rect) -> bool {
        let corners = [
            (rect.min_x, rect.min_y),
            (rect.min_x, rect.max_y),
            (rect.max_x, rect.min_y),
            (rect.max_x, rect.max_y),
        ];

        for &corner in &corners {
            if !point_in_or_on_polygon(corner, &self.points) {
                return false;
            }
        }

        let rect_edges = [
            ((rect.min_x, rect.min_y), (rect.max_x, rect.min_y)),
            ((rect.max_x, rect.min_y), (rect.max_x, rect.max_y)),
            ((rect.max_x, rect.max_y), (rect.min_x, rect.max_y)),
            ((rect.min_x, rect.max_y), (rect.min_x, rect.min_y)),
        ];

        let poly_edges = self
            .points
            .iter()
            .zip(self.points.iter().cycle().skip(1))
            .map(|(&a, &b)| (a, b));

        for (p1, p2) in poly_edges {
            for &(r1, r2) in &rect_edges {
                if segments_intersect_strict(p1, p2, r1, r2) {
                    return false;
                }
            }
        }

        true
    }
}

fn point_in_or_on_polygon(point: (f64, f64), polygon: &[(f64, f64)]) -> bool {
    let (x, y) = point;
    let mut inside = false;

    for i in 0..polygon.len() {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % polygon.len()];

        // Check if point is on the edge
        if point_on_segment(point, (x1, y1), (x2, y2)) {
            return true;
        }

        // Ray-casting algorithm for point-in-polygon test
        let intersects =
            ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1 + f64::EPSILON) + x1);
        if intersects {
            inside = !inside;
        }
    }

    inside
}

/// Check if a point lies on segment (with eps tolerance)
fn point_on_segment(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> bool {
    let (px, py) = p;
    let (ax, ay) = a;
    let (bx, by) = b;

    let cross = (bx - ax) * (py - ay) - (by - ay) * (px - ax);
    if cross.abs() > 1e-9 {
        return false;
    }

    let dot = (px - ax) * (bx - ax) + (py - ay) * (by - ay);
    if dot < 0.0 {
        return false;
    }

    let len_sq = (bx - ax).powi(2) + (by - ay).powi(2);
    dot <= len_sq
}

/// Check if two segments intersect strictly (not including touching endings)
fn segments_intersect_strict(
    p1: (f64, f64),
    p2: (f64, f64),
    q1: (f64, f64),
    q2: (f64, f64),
) -> bool {
    fn orientation(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
        (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
    }

    let o1 = orientation(p1, p2, q1);
    let o2 = orientation(p1, p2, q2);
    let o3 = orientation(q1, q2, p1);
    let o4 = orientation(q1, q2, p2);

    o1 * o2 < 0.0 && o3 * o4 < 0.0
}

#[derive(Debug, Clone)]
struct Rect {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Rect {
    fn new(a: (f64, f64), b: (f64, f64)) -> Self {
        Self {
            min_x: a.0.min(b.0),
            min_y: a.1.min(b.1),
            max_x: a.0.max(b.0),
            max_y: a.1.max(b.1),
        }
    }
}

fn parse_points_u64(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<u64>().unwrap();
            let y = y.parse::<u64>().unwrap();
            (x, y)
        })
        .collect()
}

fn parse_points_f64(input: &str) -> Vec<(f64, f64)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap())
        })
        .collect()
}

fn find_max_rectangle(input: &str, polygon: &Polygon) -> Option<(u64, (u64, u64), (u64, u64))> {
    let numbers = parse_points_u64(input);

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

    let mut max_area = 0u64;
    let mut best_rect = None;

    for &(potential_x, potential_y) in &huge_x_diff_points {
        for &(lx, ly) in &numbers {
            if lx >= *potential_x {
                continue;
            }

            for y_dir in &[1, -1] {
                let test_ly = if *y_dir > 0 {
                    (*potential_y as i64 - 1).max(ly as i64) as u64
                } else {
                    (ly as i64 + 1).min(*potential_y as i64) as u64
                };

                if test_ly == *potential_y || test_ly == 0 {
                    continue;
                }

                let rect = Rect::new(
                    (lx as f64, min(test_ly, *potential_y) as f64),
                    (*potential_x as f64, max(test_ly, *potential_y) as f64),
                );

                let area = (potential_x.abs_diff(lx) + 1) * (potential_y.abs_diff(test_ly) + 1);
                if area > max_area && polygon.contains(&rect) {
                    max_area = area;
                    best_rect = Some((area, (lx, test_ly), (*potential_x, *potential_y)));
                }
            }
        }
    }

    best_rect
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    let polygon = Polygon {
        points: parse_points_f64(input),
    };

    if let Some((area, _bl, _tr)) = find_max_rectangle(input, &polygon) {
        // dbg!(_bl, _tr);
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
}
