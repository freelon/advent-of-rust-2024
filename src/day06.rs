use std::{collections::HashSet, fs::read_to_string};

use crate::utils::grid::{Coord, Grid};

pub fn day_main() {
    let input = read_to_string("input/day06.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    let (m, pos) = parse(input);

    let dir = '^';
    let visited = get_visited(&m, pos, dir);
    visited.len()
}

fn get_visited(m: &Grid<char>, mut pos: Coord, mut dir: char) -> HashSet<Coord> {
    let mut visited = HashSet::new();
    while m.contains_key(pos) {
        let (x, y) = pos;
        visited.insert(pos);
        let next = match dir {
            '^' => (x, y - 1),
            'v' => (x, y + 1),
            '<' => (x - 1, y),
            '>' => (x + 1, y),
            _ => unreachable!(),
        };
        if m.contains_key(next) && '#' == m[next] {
            dir = match dir {
                '^' => '>',
                'v' => '<',
                '<' => '^',
                '>' => 'v',
                _ => unreachable!("asdf"),
            };
        } else {
            pos = next;
        }
    }
    visited
}

fn part2(input: &str) -> RiddleResult {
    let (m, pos) = parse(input);

    let dir = '^';
    get_visited(&m, pos, dir)
        .into_iter()
        .filter(|open| is_loop(&m, *open, pos, dir))
        .count()
}

fn parse(input: &str) -> (Grid<char>, Coord) {
    let mut m = Grid::parse(input);
    let start = m.entries().find(|(_, c)| **c == '^').unwrap().0;
    m[start] = '.';
    (m, start)
}

fn is_loop(m: &Grid<char>, block: Coord, mut pos: Coord, mut dir: char) -> bool {
    let mut visited = HashSet::new();
    loop {
        let (x, y) = pos;

        let next = match dir {
            '^' => (x, y - 1),
            'v' => (x, y + 1),
            '<' => (x - 1, y),
            '>' => (x + 1, y),
            _ => unreachable!(),
        };
        if !m.contains_key(next) {
            return false;
        } else if next == block || '#' == m[next] {
            // we only check for loops on a collision to speed things up
            if visited.contains(&(pos, dir)) {
                return true;
            }
            visited.insert((pos, dir));

            dir = match dir {
                '^' => '>',
                'v' => '<',
                '<' => '^',
                '>' => 'v',
                _ => unreachable!("asdf"),
            };
        } else {
            pos = next;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 41);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}
