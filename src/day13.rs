use std::{borrow::Borrow, fs::read_to_string};

use itertools::Itertools;
use regex::Regex;

pub fn day_main() {
    let input = read_to_string("input/day13.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = u64;

fn part1(input: &str) -> RiddleResult {
    // Button A: X+30, Y+84
    // Button B: X+74, Y+60
    // Prize: X=2358, Y=2628
    let r = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    r.captures_iter(input)
        .map(|block| {
            let (ax, ay, bx, by, px, py) = block
                .iter()
                .skip(1)
                .map(|it| it.unwrap().as_str().parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            let mut solution = None;
            for a in 0..=100 {
                for b in 0..=100 {
                    if a * ax + b * bx == px && a * ay + b * by == py {
                        let s = 3 * a + b;
                        if let Some(old) = solution {
                            if old > s {
                                solution = Some(s);
                            }
                        } else {
                            solution = Some(s);
                        }
                    }
                }
            }
            solution.unwrap_or(0)
        })
        .sum()
}

fn part2(_input: &str) -> RiddleResult {
    0
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
