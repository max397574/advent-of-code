use crate::utils::UnionFind;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    inner(input, 1000, true)
}

pub fn inner(input: &str, steps: usize, p1: bool) -> impl std::fmt::Display + use<> {
    let points = input
        .lines()
        .map(|line| {
            let (x, yz) = line.split_once(',').unwrap();
            let (y, z) = yz.split_once(',').unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            let z = z.parse::<usize>().unwrap();
            (x, y, z)
        })
        .collect::<Vec<_>>();
    let mut edges = Vec::with_capacity((points.len() - 1) * (points.len() - 1));
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            if i != j {
                let a = points[i];
                let b = points[j];
                let distance_sqr = (a.0.abs_diff(b.0)).pow(2)
                    + (a.1.abs_diff(b.1)).pow(2)
                    + (a.2.abs_diff(b.2)).pow(2);
                edges.push((distance_sqr, i, j, a.0, b.0));
            }
        }
    }
    edges.sort_unstable_by_key(|edge| edge.0);
    if p1 {
        let limit = steps.min(edges.len());
        let mut union_find = UnionFind::new(points.len());
        for edge in edges.iter().take(limit) {
            union_find.union(edge.1, edge.2);
        }
        let component_sizes = union_find.component_sizes();
        let mut component_sizes = component_sizes.values().collect::<Vec<_>>();
        component_sizes.sort_unstable();
        component_sizes
            .iter()
            .rev()
            .take(3)
            .map(|&&c| c)
            .product::<usize>()
    } else {
        let mut union_find = UnionFind::new(points.len());
        let mut i = 0;
        loop {
            let edge = edges[i];
            if union_find.union(edge.1, edge.2) && union_find.get_size(edge.1) == points.len() {
                return edge.3 * edge.4;
            }
            i += 1;
        }
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    inner(input, 1000, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689\n";

    #[test]
    fn part_1() {
        assert_eq!(inner(INPUT, 10, true).to_string(), String::from("40"))
    }

    #[test]
    fn part_2() {
        assert_eq!(inner(INPUT, 0, false).to_string(), String::from("25272"))
    }
}
