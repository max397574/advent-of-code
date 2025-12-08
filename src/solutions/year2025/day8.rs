use crate::utils::{UnionFind, parsing::ByteParsing};
use std::collections::BinaryHeap;

type Input = (
    Vec<(isize, isize, isize)>,
    BinaryHeap<(isize, usize, usize)>,
);

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    innerp1(parse(input), 1000)
}

fn innerp1(input: Input, steps: usize) -> usize {
    let (points, mut edges) = input;
    let limit = steps.min(edges.len());
    let mut union_find = UnionFind::new(points.len());
    for _ in 0..limit {
        let edge = edges.pop().unwrap();
        union_find.union(edge.1, edge.2);
    }
    let component_sizes: std::collections::HashMap<usize, usize> = {
        let this = &mut union_find;
        let mut sizes = std::collections::HashMap::new();
        for i in 0..this.parent.len() {
            let root = this.find(i);
            sizes.insert(root, this.size[root]);
        }
        sizes.into_iter().collect()
    };
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    for cmp_sz in component_sizes.values() {
        if *cmp_sz > max1 {
            max3 = max2;
            max2 = max1;
            max1 = *cmp_sz;
        } else if *cmp_sz > max2 {
            max3 = max2;
            max2 = *cmp_sz;
        } else {
            max3 = max3.max(*cmp_sz);
        }
    }
    max1 * max2 * max3
}
pub fn part2(input: &str) -> impl std::fmt::Display + use<> {
    innerp2(parse(input))
}

pub fn innerp2(input: Input) -> impl std::fmt::Display + use<> {
    let (points, mut edges) = input;
    let mut union_find = UnionFind::new(points.len());
    loop {
        let edge = edges.pop().unwrap();
        if union_find.union(edge.1, edge.2) && union_find.get_size(edge.1) == points.len() {
            return points[edge.1].0 * points[edge.2].0;
        }
    }
}

pub fn parse(input: &str) -> Input {
    let points = input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let (x, yz) = line.split_once(|&c| c == b',').unwrap();
            let (y, z) = yz.split_once(|&c| c == b',').unwrap();
            let x = x.as_num::<isize>();
            let y = y.as_num::<isize>();
            let z = z.as_num::<isize>();
            (x, y, z)
        })
        .collect::<Vec<_>>();
    let mut edges = BinaryHeap::with_capacity(((points.len()) * (points.len() - 1)) >> 1);
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let a = points[i];
            let b = points[j];
            let distance_sqr = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2);
            edges.push((-distance_sqr, i, j));
        }
    }
    (points, edges)
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
        assert_eq!(innerp1(parse(INPUT), 10).to_string(), String::from("40"))
    }

    #[test]
    fn part_2() {
        assert_eq!(innerp2(parse(INPUT)).to_string(), String::from("25272"))
    }
}
