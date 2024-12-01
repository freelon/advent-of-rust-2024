use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day01.txt").unwrap();
    let part1 = part1(&input);
    println!("part1: {}", part1);
}

type Int = i32;

fn part1(input: &str) -> Int {
    let (mut l, mut r) = input
        .trim()
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(x, y)| (x.parse::<Int>().unwrap(), y.parse::<Int>().unwrap()))
        .fold((vec![], vec![]), |(mut v1, mut v2), (a, b)| {
            v1.push(a);
            v2.push(b);
            (v1, v2)
        });

    l.sort();
    r.sort();

    l.into_iter()
        .zip(r.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[cfg(test)]
const TEST_INPUT: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3
";

#[test]
fn test() {
    assert_eq!(part1(TEST_INPUT), 11);
}
