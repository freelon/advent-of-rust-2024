use itertools::Itertools;
use std::fs::read_to_string;

pub fn day_main() {
    let input = read_to_string("input/day01.txt").unwrap();
    println!(" part1: {}", part1(&input));
    println!(" part2: {}", part2(&input));
}

type Int = i32;

fn part1(input: &str) -> Int {
    let (mut l, mut r) = make_lists(input);

    l.sort();
    r.sort();

    l.into_iter().zip(r).map(|(l, r)| (l - r).abs()).sum()
}

fn part2(input: &str) -> Int {
    let (l, r) = make_lists(input);

    l.iter()
        .map(|l| r.iter().filter(|r| *r == l).count() as Int * l)
        .sum()
}

fn make_lists(input: &str) -> (Vec<Int>, Vec<Int>) {
    input
        .trim()
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(x, y)| (x.parse::<Int>().unwrap(), y.parse::<Int>().unwrap()))
        .fold((vec![], vec![]), |(mut v1, mut v2), (a, b)| {
            v1.push(a);
            v2.push(b);
            (v1, v2)
        })
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 11);
    }
    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 31);
    }
}
