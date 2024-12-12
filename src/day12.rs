use std::{collections::HashSet, fs::read_to_string};

use crate::utils::grid::{Coord, Grid};

pub fn day_main() {
    let input = read_to_string("input/day12.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = u64;

const NEIGHBORS: &[Coord] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

fn part1(input: &str) -> RiddleResult {
    let garden = Grid::parse(input);
    let mut processed: HashSet<Coord> = HashSet::new();
    let mut result = 0;
    for (p, c) in garden.entries() {
        if processed.contains(&p) {
            continue;
        }
        let mut area = 0;
        let mut fence = 0;
        let mut further = vec![p];
        while let Some(current) = further.pop() {
            if processed.contains(&current) {
                continue;
            }
            area += 1;
            for next in NEIGHBORS.iter().map(|d| (current.0 + d.0, current.1 + d.1)) {
                let next_c = garden.get(next);
                if next_c.is_none() || next_c.unwrap() != c {
                    fence += 1;
                } else {
                    further.push(next);
                }
            }
            processed.insert(current);
        }
        result += area * fence;
    }
    result
}

fn part2(_input: &str) -> RiddleResult {
    0
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"AAAA
BBCD
BBCC
EEEC
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 140);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
