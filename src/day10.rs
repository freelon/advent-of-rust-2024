use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

use crate::utils::grid::Grid;

pub fn day_main() {
    let input = read_to_string("input/day10.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input)); // 832 wrong
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    let grid = Grid::parse(input);
    // println!("{grid:?}");
    // println!("----");
    let m: HashMap<(i64, i64), u32> = HashMap::from_iter(grid.entries().map(|((x, y), c)| {
        (
            (x, y),
            if *c == '.' {
                1000
            } else {
                c.to_digit(10).unwrap()
            },
        )
    }));
    let grid = Grid::from(m);
    // println!("{grid:?}");
    let trail_heads = grid.entries().filter(|(_, h)| **h == 0).collect_vec();
    let mut result = 0;
    for (th, th_height) in trail_heads {
        let mut count = 0;
        let mut visited = HashSet::new();
        let mut queue = vec![(th, th_height)];
        while let Some((p, h)) = queue.pop() {
            if visited.contains(&(p, h)) {
                continue;
            }
            visited.insert((p, h));

            if *h == 9 {
                count += 1;
                // println!("found {p:?}");
                continue;
            }

            let nexts = vec![
                (p.0, p.1 + 1),
                (p.0, p.1 - 1),
                (p.0 + 1, p.1),
                (p.0 - 1, p.1),
            ];

            for next in nexts {
                if let Some(nh) = grid.get(next) {
                    if *nh == h + 1 {
                        queue.push((next, nh));
                        // println!("going from {:?} to {:?}", (p, h), (next, nh));
                    }
                }
            }
        }
        // println!("from {th:?} found {count}");
        result += count;
    }
    result
}

fn part2(_input: &str) -> RiddleResult {
    0
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 36);
    }
    #[test]
    fn test1a() {
        assert_eq!(
            part1(
                r"0123
1234
8765
9876
"
            ),
            1
        );
    }

    #[test]
    fn test1b() {
        assert_eq!(
            part1(
                r"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
"
            ),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
