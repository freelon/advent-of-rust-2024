use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;

use crate::utils::grid::Grid;

pub fn day_main() {
    let input = read_to_string("input/day08.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    let grid = Grid::parse(input);
    let mut antinodes = HashSet::new();
    let antennas = grid
        .entries()
        .filter(|(_, c)| **c != '.')
        .into_group_map_by(|(_, c)| *c);
    for (_, coords) in antennas {
        for (a, _) in coords.iter() {
            for (b, _) in coords.iter() {
                if a == b {
                    continue;
                }
                let dx = a.0 - b.0;
                let dy = a.1 - b.1;
                let p = (a.0 + dx, a.1 + dy);
                if grid.contains_key(p) {
                    antinodes.insert(p);
                }
            }
        }
    }
    antinodes.len()
}

fn part2(input: &str) -> RiddleResult {
    let grid = Grid::parse(input);
    let mut antinodes = HashSet::new();
    let antennas = grid
        .entries()
        .filter(|(_, c)| **c != '.')
        .into_group_map_by(|(_, c)| *c);
    for (_, coords) in antennas {
        for (a, _) in coords.iter() {
            for (b, _) in coords.iter() {
                if a == b {
                    continue;
                }
                antinodes.insert(*a);
                antinodes.insert(*b);
                let dx = a.0 - b.0;
                let dy = a.1 - b.1;
                for i in 1.. {
                    let p = (a.0 + i * dx, a.1 + i * dy);
                    if grid.contains_key(p) {
                        antinodes.insert(p);
                    } else {
                        break;
                    }
                }
                for i in 1.. {
                    let p = (a.0 - i * dx, a.1 - i * dy);
                    if grid.contains_key(p) {
                        antinodes.insert(p);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 14);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 34);
    }
}
