use std::fs::read_to_string;

use itertools::Itertools;

pub fn day_main() {
    let input = read_to_string("input/day25.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
}

type RiddleResult = i64;

fn part1(input: &str) -> RiddleResult {
    let (locks, keys): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .map(|item| {
            let block = item
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec();
            let lock = block[0].contains(&'#');
            let summary: (u8, u8, u8, u8, u8) = if lock {
                // lock
                (0..5)
                    .map(|x| {
                        (0..6)
                            .map(|y| block[y][x])
                            .enumerate()
                            .find(|(_, it)| *it == '.')
                            .unwrap_or((6, '.'))
                            .0 as u8
                    })
                    .collect_tuple()
                    .unwrap()
            } else {
                (0..5)
                    .map(|x| {
                        7 - ((0..6)
                            .map(|y| block[y][x])
                            .enumerate()
                            .find(|(_, it)| *it == '#')
                            .unwrap_or((6, '.'))
                            .0 as u8)
                    })
                    .collect_tuple()
                    .unwrap()
            };
            (lock, summary.0, summary.1, summary.2, summary.3, summary.4)
        })
        .partition(|s| s.0);

    let mut matches = 0;
    for lock in locks {
        for &key in &keys {
            if lock.1 + key.1 < 8
                && lock.2 + key.2 < 8
                && lock.3 + key.3 < 8
                && lock.4 + key.4 < 8
                && lock.5 + key.5 < 8
            {
                matches += 1;
            }
        }
    }
    matches
}

#[cfg(test)]
mod test {
    use super::part1;

    const TEST_INPUT: &str = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
    ";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }
}
