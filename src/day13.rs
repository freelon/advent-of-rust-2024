use std::fs::read_to_string;

use itertools::Itertools;
use regex::Regex;

pub fn day_main() {
    let input = read_to_string("input/day13.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = i64;

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
                .map(|it| it.unwrap().as_str().parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            solve((ax, ay, bx, by, px, py)).unwrap_or(0)
        })
        .sum()
}

fn part2(input: &str) -> RiddleResult {
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
                .map(|it| it.unwrap().as_str().parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            let solution = solve((ax, ay, bx, by, px + 10000000000000, py + 10000000000000));
            solution.unwrap_or(0)
        })
        .sum()
}

fn solve((ax, ay, bx, by, px, py): (i64, i64, i64, i64, i64, i64)) -> Option<RiddleResult> {
    let b = (ay * px - ax * py) / (ay * bx - ax * by);
    let a = (px - b * bx) / ax;
    if a >= 0 && b >= 0 && a * ax + b * bx == px && a * ay + b * by == py {
        Some(3 * a + b)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 480);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 875318608908);
    }
}
