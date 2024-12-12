use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;

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

fn part2(input: &str) -> RiddleResult {
    let garden = Grid::parse(input);
    let mut processed: HashSet<Coord> = HashSet::new();
    let mut result = 0;
    for (p, c) in garden.entries() {
        if processed.contains(&p) {
            continue;
        }
        let mut area = 0;
        let mut further = vec![p];
        let mut convex_corners = 0;
        let mut concave_corners = 0;
        while let Some(current) = further.pop() {
            if processed.contains(&current) {
                continue;
            }
            area += 1;
            for next in NEIGHBORS.iter().map(|d| (current.0 + d.0, current.1 + d.1)) {
                let next_c = garden.get(next);
                if next_c.is_none() || next_c.unwrap() != c {
                } else {
                    further.push(next);
                }
            }
            processed.insert(current);
            for (da, db) in NEIGHBORS.iter().circular_tuple_windows() {
                let a = (current.0 + da.0, current.1 + da.1);
                let b = (current.0 + db.0, current.1 + db.1);
                if garden.get(a) != Some(c) && garden.get(b) != Some(c) {
                    convex_corners += 1;
                } else if garden.get(a) == Some(c)
                    && garden.get(b) == Some(c)
                    && garden.get((current.0 + da.0 + db.0, current.1 + da.1 + db.1)) != Some(c)
                {
                    concave_corners += 1;
                }
            }
        }
        let var_name = convex_corners + concave_corners;
        let var_name = area * (var_name);
        result += var_name;
    }
    result
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
        assert_eq!(part2(TEST_INPUT), 80);
    }
}
