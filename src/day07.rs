use std::fs::read_to_string;

use itertools::Itertools;
use regex::Regex;

pub fn day_main() {
    let input = read_to_string("input/day07.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = i64;

fn part1(input: &str) -> RiddleResult {
    let regex = Regex::new("(\\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            regex
                .find_iter(line)
                .map(|capture| capture.as_str().parse::<i64>().unwrap())
                .collect_vec()
        })
        .filter(|v| well_calibrated(v))
        .map(|v| v[0])
        .sum()
}

fn well_calibrated(list: &[i64]) -> bool {
    let expected = list[0];
    let operands = list.iter().skip(1).copied().collect_vec();
    let operators = ops(operands.len() - 1);
    for ops in operators.into_iter() {
        let v = ops
            .into_iter()
            .zip(operands.iter().skip(1))
            .fold(operands[0], |acc, (op, val)| match op {
                '*' => acc * val,
                '+' => acc + val,
                _ => panic!(),
            });
        if v == expected {
            return true;
        }
    }
    false
}

fn ops(n: usize) -> Vec<Vec<char>> {
    if n == 1 {
        return vec![vec!['+'], vec!['*']];
    }
    let v = ops(n - 1);
    v.into_iter()
        .flat_map(|vector| {
            let mut v1 = vector.clone();
            let mut v2 = vector;
            v1.push('+');
            v2.push('*');
            [v1, v2]
        })
        .collect_vec()
}

fn part2(input: &str) -> RiddleResult {
    let regex = Regex::new("(\\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            regex
                .find_iter(line)
                .map(|capture| capture.as_str().parse::<i64>().unwrap())
                .collect_vec()
        })
        .filter(|v| well(v[0], v[1], &v[2..]))
        .map(|v| v[0])
        .sum()
}

fn well(expected: i64, acc: i64, operands: &[i64]) -> bool {
    if operands.is_empty() {
        return expected == acc;
    }
    if acc > expected {
        return false;
    }

    let next = operands[0];
    let remainder = &operands[1..];

    let v_mul = acc * next;
    let v_plus = acc + next;
    let v_concat = concat(acc, next);
    well(expected, v_mul, remainder)
        || well(expected, v_plus, remainder)
        || well(expected, v_concat, remainder)
}

fn concat(lhs: i64, rhs: i64) -> i64 {
    let exp: u32 = (rhs as f64 + 1.0).log10().ceil() as u32;
    let shift = 10i64.pow(exp);
    lhs * shift + rhs
}

#[cfg(test)]
mod test {
    use super::{concat, part1, part2};

    const TEST_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 3749);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 11387);
    }

    #[test]
    fn test_concat() {
        assert_eq!(1234, concat(12, 34));
        assert_eq!(1210, concat(12, 10));
    }
}
