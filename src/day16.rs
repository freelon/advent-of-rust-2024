use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

use crate::utils::grid::{Coord, Grid};

pub fn day_main() {
    let input = read_to_string("input/day16.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn nexts(&self) -> [Self; 2] {
        match self {
            Dir::N | Dir::S => [Dir::W, Dir::E],
            Dir::E | Dir::W => [Dir::S, Dir::N],
        }
    }
}

struct Step(Coord, Dir, usize, Option<Node>);

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
    }
}
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        other.2.eq(&self.2)
    }
}
impl Eq for Step {}

fn part1(input: &str) -> RiddleResult {
    use Dir::*;
    let maze = Grid::parse(input);
    let start = maze.entries().find(|(_p, c)| **c == 'S').unwrap().0;
    let end = maze.entries().find(|(_p, c)| **c == 'E').unwrap().0;
    let mut visited = HashMap::<(Coord, Dir), usize>::new();
    let mut stack: BinaryHeap<Step> = BinaryHeap::new();
    stack.push(Step(start, E, 0, None));
    while let Some(Step(np, nd, cost, _)) = stack.pop() {
        if visited.contains_key(&(np, nd)) {
            continue;
        }
        visited.insert((np, nd), cost);
        if np == end {
            return cost;
        }
        for d in nd.nexts() {
            if !visited.contains_key(&(np, d)) {
                stack.push(Step(np, d, cost + 1000, None));
            }
        }
        let forward = match nd {
            N => (np.0, np.1 - 1),
            E => (np.0 + 1, np.1),
            S => (np.0, np.1 + 1),
            W => (np.0 - 1, np.1),
        };
        if maze[forward] != '#' && !visited.contains_key(&(forward, nd)) {
            stack.push(Step(forward, nd, cost + 1, None));
        }
    }
    panic!("no path found")
}

type Node = (Coord, Dir);

fn part2(input: &str) -> RiddleResult {
    use Dir::*;
    let maze = Grid::parse(input);
    let start = maze.entries().find(|(_p, c)| **c == 'S').unwrap().0;
    let end = maze.entries().find(|(_p, c)| **c == 'E').unwrap().0;
    let mut visited = HashMap::<Node, (usize, Vec<Node>)>::new();
    let mut stack: BinaryHeap<Step> = BinaryHeap::new();
    stack.push(Step(start, E, 0, None));

    let mut best: Option<usize> = None;
    while let Some(Step(np, nd, cost, pred)) = stack.pop() {
        if let Some(b) = best {
            if b < cost {
                break; // can't reach the end point with best cost anymore
            }
        }
        let entry = visited.entry((np, nd)).or_insert_with(|| {
            (
                cost,
                if let Some(pred) = pred {
                    vec![pred]
                } else {
                    vec![]
                },
            )
        });
        if entry.0 < cost {
            continue;
        }
        if let Some(pred) = pred {
            entry.1.push(pred);
        }
        if np == end {
            best = Some(cost);
        }
        for d in nd.nexts() {
            if !visited.contains_key(&(np, d)) {
                stack.push(Step(np, d, cost + 1000, Some((np, nd))));
            }
        }
        let forward = match nd {
            N => (np.0, np.1 - 1),
            E => (np.0 + 1, np.1),
            S => (np.0, np.1 + 1),
            W => (np.0 - 1, np.1),
        };
        if maze[forward] != '#' && !visited.contains_key(&(forward, nd)) {
            stack.push(Step(forward, nd, cost + 1, Some((np, nd))));
        }
    }
    let mut accounted = HashSet::<Node>::new();
    let mut stack: Vec<Node> = visited
        .iter()
        .filter(|((pos, _dir), _)| *pos == end)
        .map(|(node, _)| *node)
        .to_owned()
        .collect_vec();
    while let Some(node) = stack.pop() {
        if accounted.contains(&node) {
            continue;
        }
        accounted.insert(node);
        for pred in visited[&node].1.iter() {
            stack.push(*pred);
        }
    }
    accounted.into_iter().map(|(pos, _)| pos).unique().count()
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 7036);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 45);
    }
}
