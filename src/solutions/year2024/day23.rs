use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let mut connections = HashMap::new();
    input.lines().for_each(|line| {
        let first = &line[0..2];
        let second = &line[3..5];
        connections.entry(first).or_insert(vec![]).push(second);
        connections.entry(second).or_insert(vec![]).push(first);
    });

    let mut cycles = HashSet::new();

    for &node in connections.keys() {
        for &neighbor1 in &connections[node] {
            for &neighbor2 in &connections[node] {
                if neighbor1 != neighbor2
                    && connections[neighbor1].contains(&neighbor2)
                    && (node.starts_with('t')
                        || neighbor1.starts_with('t')
                        || neighbor2.starts_with('t'))
                {
                    let mut cycle = vec![node, neighbor1, neighbor2];
                    cycle.sort();
                    cycles.insert(cycle);
                }
            }
        }
    }

    cycles.len()
}

fn bron_kerbosch<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    max_clique: &mut HashSet<&'a str>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            *max_clique = r.clone();
        }
        return;
    }

    let p_copy = p.clone();
    for &v in p_copy.iter() {
        let neighbors: HashSet<&str> = graph[v].iter().cloned().collect();

        r.insert(v);
        let mut new_p: HashSet<_> = p.intersection(&neighbors).cloned().collect();
        let mut new_x: HashSet<_> = x.intersection(&neighbors).cloned().collect();

        bron_kerbosch(graph, r, &mut new_p, &mut new_x, max_clique);

        r.remove(v);
        p.remove(v);
        x.insert(v);
    }
}

fn find_maximum_clique<'a>(graph: &'a HashMap<&'a str, Vec<&'a str>>) -> HashSet<&'a str> {
    let mut r = HashSet::new();
    let mut p: HashSet<_> = graph.keys().cloned().collect();
    let mut x = HashSet::new();
    let mut max_clique = HashSet::new();

    bron_kerbosch(graph, &mut r, &mut p, &mut x, &mut max_clique);

    max_clique
}

pub fn part2(input: &str) -> String {
    let mut graph = HashMap::new();
    input.lines().for_each(|line| {
        let first = &line[0..2];
        let second = &line[3..5];
        graph.entry(first).or_insert(vec![]).push(second);
        graph.entry(second).or_insert(vec![]).push(first);
    });

    let clique = find_maximum_clique(&graph);
    let mut clique: Vec<_> = clique.into_iter().collect();
    clique.sort();
    let mut out = String::new();
    let mut iter = clique.iter();
    out.push_str(iter.next().unwrap());
    iter.for_each(|v| {
        out.push(',');
        out.push_str(v);
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), String::from("7"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part2(INPUT).to_string(), String::from("co,de,ka,ta"))
    }
}
