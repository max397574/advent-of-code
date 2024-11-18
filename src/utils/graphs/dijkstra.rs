use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<T> {
    cost: usize,
    node: T,
}

impl<T> Ord for State<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<T> PartialOrd for State<T>
where
    T: Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path<T>(
    start: T,
    adjacency: &dyn Fn(T) -> Vec<T>,
    get_cost: &dyn Fn(T) -> usize,
    is_end: &dyn Fn(T) -> bool,
) -> Option<usize>
where
    T: Eq + PartialEq + Ord + Clone + Copy + Hash,
{
    let mut dist_map: HashMap<T, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist_map.insert(start, 0);

    heap.push(State {
        cost: 0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        if is_end(node) {
            return Some(cost);
        }

        if cost > *dist_map.get(&node).unwrap_or(&usize::MAX) {
            continue;
        }

        for neighbour in &adjacency(node) {
            let next = State {
                cost: cost + get_cost(*neighbour),
                node: *neighbour,
            };
            if next.cost < *dist_map.get(&next.node).unwrap_or(&usize::MAX) {
                heap.push(next);
                *dist_map.entry(next.node).or_insert(usize::MAX) = next.cost;
            }
        }
    }

    None
}
