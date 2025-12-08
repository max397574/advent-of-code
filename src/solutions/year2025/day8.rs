use crate::utils::{UnionFind, parsing::ByteParsing};
use std::collections::BinaryHeap;

pub fn part1(input: &str) -> impl std::fmt::Display + use<> {
    unsafe {
        let mut points = [(0, 0, 0); 1000];
        input.lines().enumerate().for_each(|(i, line)| {
            let line = line.as_bytes();
            let mut idx = 0;
            let mut x = 0;
            while *line.get_unchecked(idx) >= b'0' {
                x = x * 10 + (*line.get_unchecked(idx) - b'0') as i64;
                idx += 1;
            }
            idx += 1;
            let mut y = 0;
            while *line.get_unchecked(idx) >= b'0' {
                y = y * 10 + (*line.get_unchecked(idx) - b'0') as i64;
                idx += 1;
            }
            idx += 1;
            let mut z = 0;
            while *line.get_unchecked(idx) >= b'0' {
                z = z * 10 + (*line.get_unchecked(idx) - b'0') as i64;
                idx += 1;
            }
            *points.get_unchecked_mut(i) = (x, y, z);
        });

        let mut edges = BinaryHeap::with_capacity(1000);
        let a = points.get_unchecked(0);
        edges.extend((1..1000).map(|j| {
            let b = *points.get_unchecked(j);
            let dx = a.0 - b.0;
            let dy = a.1 - b.1;
            let dz = a.2 - b.2;
            let dist_sqr = dx * dx + dy * dy + dz * dz;
            (dist_sqr, 0, j)
        }));
        let a = points.get_unchecked(1);
        let b = points.get_unchecked(2);
        let distance_sqr = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2);
        edges.push((distance_sqr, 1, 2));
        for i in 1..1000 {
            let a = points.get_unchecked(i);
            for j in (i + 1)..1000 {
                let b = points.get_unchecked(j);
                let dx = a.0 - b.0;
                let dy = a.1 - b.1;
                let dz = a.2 - b.2;
                let dist_sqr = dx * dx + dy * dy + dz * dz;
                let mut max = edges.peek_mut().unwrap_unchecked();
                if dist_sqr < max.0 {
                    *max = (dist_sqr, i, j);
                }
            }
        }
        let mut parent = [0; 1000];
        let mut size = [1; 1000];
        for (i, p) in parent.iter_mut().enumerate() {
            *p = i;
        }

        pub fn find(mut x: usize, parent: &mut [usize; 1000]) -> usize {
            unsafe {
                while *parent.get_unchecked(x) != x {
                    *parent.get_unchecked_mut(x) = *parent.get_unchecked(*parent.get_unchecked(x));
                    x = *parent.get_unchecked(x);
                }
            }
            x
        }

        pub fn union(
            x: usize,
            y: usize,
            parent: &mut [usize; 1000],
            size: &mut [u32; 1000],
        ) -> bool {
            let root_x = find(x, parent);
            let root_y = find(y, parent);
            if root_x == root_y {
                return false;
            }
            unsafe {
                if *size.get_unchecked(root_x) < *size.get_unchecked(root_y) {
                    *parent.get_unchecked_mut(root_x) = root_y;
                    *size.get_unchecked_mut(root_y) += *size.get_unchecked(root_x);
                } else {
                    *parent.get_unchecked_mut(root_y) = root_x;
                    *size.get_unchecked_mut(root_x) += *size.get_unchecked(root_y);
                }
            }
            true
        }

        for edge in edges.iter() {
            union(edge.1, edge.2, &mut parent, &mut size);
        }

        let mut max1 = 0;
        let mut max2 = 0;
        let mut max3 = 0;
        for i in 0..1000 {
            // only look at roots
            if *parent.get_unchecked(i) == i {
                let cmp_sz = *size.get_unchecked(i);
                if cmp_sz > max1 {
                    max3 = max2;
                    max2 = max1;
                    max1 = cmp_sz;
                } else if cmp_sz > max2 {
                    max3 = max2;
                    max2 = cmp_sz;
                } else {
                    max3 = max3.max(cmp_sz);
                }
            }
        }
        max1 * max2 * max3
    }
}

type Input = (
    [(isize, isize, isize); 1000],
    BinaryHeap<(isize, usize, usize)>,
);

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
    unsafe {
        let mut points = [(0, 0, 0); 1000];
        input.lines().take(1000).enumerate().for_each(|(i, line)| {
            let line = line.as_bytes();
            let (x, yz) = line.split_once(|&c| c == b',').unwrap_unchecked();
            let (y, z) = yz.split_once(|&c| c == b',').unwrap_unchecked();
            let x = x.as_num::<isize>();
            let y = y.as_num::<isize>();
            let z = z.as_num::<isize>();
            *points.get_unchecked_mut(i) = (x, y, z);
        });
        let mut edges = BinaryHeap::with_capacity(((points.len()) * (points.len() - 1)) >> 1);
        for i in 0..points.len() {
            let a = points.get_unchecked(i);
            for j in (i + 1)..points.len() {
                let b = points.get_unchecked(j);
                let distance_sqr = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2);
                edges.push((-distance_sqr, i, j));
            }
        }
        (points, edges)
    }
}

pub fn parsep1(input: &str) -> Input {
    unsafe {
        let mut points = [(0, 0, 0); 1000];
        input.lines().take(1000).enumerate().for_each(|(i, line)| {
            let line = line.as_bytes();
            let (x, yz) = line.split_once(|&c| c == b',').unwrap_unchecked();
            let (y, z) = yz.split_once(|&c| c == b',').unwrap_unchecked();
            let x = x.as_num::<isize>();
            let y = y.as_num::<isize>();
            let z = z.as_num::<isize>();
            *points.get_unchecked_mut(i) = (x, y, z);
        });
        let mut edges = BinaryHeap::with_capacity(1000);
        let a = points.get_unchecked(0);
        for j in 1..points.len() {
            let b = points.get_unchecked(j);
            let distance_sqr = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2);
            edges.push((distance_sqr, 0, j));
        }
        let a = points.get_unchecked(1);
        let b = points.get_unchecked(2);
        let distance_sqr = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2);
        edges.push((distance_sqr, 0, 2));
        for i in 1..points.len() {
            for j in (i + 1)..points.len() {
                let a = points.get_unchecked(i);
                let b = points.get_unchecked(j);
                let distance_sqr = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2);
                if distance_sqr < edges.peek().unwrap().0 {
                    edges.pop();
                    edges.push((distance_sqr, i, j));
                }
            }
        }
        (points, edges)
    }
}
