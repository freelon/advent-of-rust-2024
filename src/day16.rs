use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

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

struct Step(Coord, Dir, usize);

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
    stack.push(Step(start, E, 0));
    while let Some(Step(np, nd, cost)) = stack.pop() {
        if visited.contains_key(&(np, nd)) {
            continue;
        }
        visited.insert((np, nd), cost);
        if np == end {
            return cost;
        }
        for d in nd.nexts() {
            if !visited.contains_key(&(np, d)) {
                stack.push(Step(np, d, cost + 1000));
            }
        }
        let forward = match nd {
            N => (np.0, np.1 - 1),
            E => (np.0 + 1, np.1),
            S => (np.0, np.1 + 1),
            W => (np.0 - 1, np.1),
        };
        if maze[forward] != '#' && !visited.contains_key(&(forward, nd)) {
            stack.push(Step(forward, nd, cost + 1));
        }
    }
    panic!("no path found")
}

fn part2(_input: &str) -> RiddleResult {
    0
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
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
