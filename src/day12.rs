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
        let mut group = HashSet::new();
        while let Some(current) = further.pop() {
            if processed.contains(&current) {
                continue;
            }
            group.insert(current);
            area += 1;
            for next in NEIGHBORS.iter().map(|d| (current.0 + d.0, current.1 + d.1)) {
                let next_c = garden.get(next);
                if next_c.is_none() || next_c.unwrap() != c {
                } else {
                    further.push(next);
                }
            }
            processed.insert(current);
        }
        result += area * sides(group);
    }
    result
}

fn sides(group: HashSet<(i64, i64)>) -> RiddleResult {
    let x_range = group.iter().min_by_key(|it| it.0).unwrap().0
        ..=group.iter().max_by_key(|it| it.0).unwrap().0;
    let y_range = group.iter().min_by_key(|it| it.1).unwrap().1
        ..=group.iter().max_by_key(|it| it.1).unwrap().1;
    let mut result = 0;
    for x in x_range {
        // 1 for the ending of the last group, +1 for each time a line "stops", e.g. between the current point and next there is a gap
        let points = group
            .iter()
            .filter(|p| p.0 == x)
            .filter(|p| !group.contains(&(p.0 - 1, p.1)))
            .sorted_by_key(|p| p.1)
            .collect_vec();
        if points.is_empty() {
        } else {
            let count = points
                .into_iter()
                .tuple_windows()
                .filter(|(a, b)| a.1 + 1 != b.1)
                .count();
            result += 1 + count;
        }
        let points = group
            .iter()
            .filter(|p| p.0 == x)
            .filter(|p| !group.contains(&(p.0 + 1, p.1)))
            .sorted_by_key(|p| p.1)
            .collect_vec();
        if points.is_empty() {
        } else {
            let count = points
                .into_iter()
                .tuple_windows()
                .filter(|(a, b)| a.1 + 1 != b.1)
                .count();
            result += 1 + count;
        }
    }
    for y in y_range {
        let points = group
            .iter()
            .filter(|p| p.1 == y)
            .filter(|p| !group.contains(&(p.0, p.1 - 1)))
            .sorted_by_key(|p| p.0)
            .collect_vec();
        if points.is_empty() {
        } else {
            let count = points
                .into_iter()
                .tuple_windows()
                .filter(|(a, b)| a.0 + 1 != b.0)
                .count();
            result += 1 + count;
        }
        let points = group
            .iter()
            .filter(|p| p.1 == y)
            .filter(|p| !group.contains(&(p.0, p.1 + 1)))
            .sorted_by_key(|p| p.0)
            .collect_vec();
        if points.is_empty() {
        } else {
            let count = points
                .into_iter()
                .tuple_windows()
                .filter(|(a, b)| a.0 + 1 != b.0)
                .count();
            result += 1 + count;
        }
    }
    result as RiddleResult
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::day12::sides;

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

    #[test]
    fn sides_one() {
        let group = HashSet::from_iter([(3, 5)]);
        assert_eq!(sides(group), 4);
    }

    #[test]
    fn sides_plus() {
        let group = HashSet::from_iter([(3, 3), (3, 4), (3, 2), (2, 3), (4, 3)]);
        assert_eq!(sides(group), 12);
    }
}
