use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

pub fn day_main() {
    let input = read_to_string("input/day23.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    let neighbours = make_graph(input);
    let mut sets = vec![];
    for a in neighbours.keys() {
        for b in &neighbours[a] {
            let common = neighbours[a].intersection(&neighbours[b]);
            for c in common {
                let mut v = vec![a, b, c];
                v.sort_unstable();
                sets.push(v);
            }
        }
    }
    let sets = sets.iter().unique().collect_vec();
    dbg!(sets.len());
    sets.iter()
        .filter(|set| set.iter().any(|t| t.starts_with("t")))
        .count()
}

fn make_graph(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut neighbours: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let (a, b) = line.split_once("-").unwrap();
        neighbours
            .entry(a)
            .and_modify(|list| {
                list.insert(b);
            })
            .or_insert_with(|| HashSet::from([b]));
        neighbours
            .entry(b)
            .and_modify(|list| {
                list.insert(a);
            })
            .or_insert_with(|| HashSet::from([a]));
    });
    neighbours
}

fn part2(input: &str) -> String {
    let graph = make_graph(input);

    let mut best = "".to_string();
    for a in graph.keys() {
        let mut edge_counts = HashMap::<&str, u32>::new();
        for b in &graph[a] {
            edge_counts.entry(b).and_modify(|v| *v += 1).or_insert(1);
            for c in &graph[b] {
                edge_counts.entry(c).and_modify(|v| *v += 1).or_insert(1);
            }
        }
        // now we have all counts for shared edges
        // next, we find the highest number k for which there are k nodes which are connected to at least k other nodes
        let mut k = *edge_counts.values().max().unwrap();
        while k > 0 {
            let nodes = edge_counts
                .iter()
                .filter(|(_, v)| **v >= k)
                .map(|(k, _)| k)
                .collect_vec();

            if nodes.len() < (k + 1) as usize {
                k -= 1;
                continue;
            }

            // now we check if indeed all of the candidate nodes are connected
            let found = nodes
                .iter()
                .filter(|&x| nodes.iter().all(|y| y == x || graph[*x].contains(*y)))
                .sorted_unstable()
                .join(",");
            // disclaimer: this works for this input but not in general (think of a graph where a is in the middle and the other nodes in
            // a circle around it, each connected to their left and right neighbour. There could be 10 nodes with degree 3 in there
            // but ofc they wouldn't be all connected - so this "solution" would return nothing, althoug a max clique of
            // size 3 would exist)

            if found.len() > best.len() {
                best = found;
            }
            break;
        }
    }

    best
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"kh-tc
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
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), "co,de,ka,ta");
    }
}
