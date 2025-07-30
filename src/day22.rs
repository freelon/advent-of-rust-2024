use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::BitXor,
};

use itertools::Itertools;

pub fn day_main() {
    let input = read_to_string("input/day22.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = i64;

fn part1(input: &str) -> RiddleResult {
    input.lines().map(|is| solution(is.parse().unwrap())).sum()
}

fn solution(initial_secret: i64) -> i64 {
    let mut secret = initial_secret;
    for _ in 0..2000 {
        secret = fun_name(secret);
    }
    secret
}

fn fun_name(secret: i64) -> i64 {
    let secret = prune(mix(secret * 64, secret));
    let secret = prune(mix(secret / 32, secret));
    prune(mix(secret * 2048, secret))
}

fn mix(value: i64, secret: i64) -> i64 {
    value.bitxor(secret)
}

fn prune(value: i64) -> i64 {
    value % 16777216
}

fn part2(input: &str) -> RiddleResult {
    let deltas = input
        .trim()
        .lines()
        .map(|is| is.parse().unwrap())
        .map(|s| generate(s))
        .collect_vec();

    let best = deltas
        .iter()
        .map(|monkey| {
            let mut result: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

            for (a, b, c, d) in monkey.iter().tuple_windows() {
                let t = (a.delta, b.delta, c.delta, d.delta);
                if !result.contains_key(&t) {
                    result.insert(t, d.price);
                }
            }
            result
        })
        .collect_vec();

    let mut all_quadruples = HashSet::new();
    for map in &best {
        for &seq in map.keys() {
            all_quadruples.insert(seq);
        }
    }
    println!(
        "we have {} different sequences to check",
        all_quadruples.len()
    );

    all_quadruples
        .into_iter()
        .enumerate()
        .map(|(n, seq)| {
            if n % 100 == 0 {
                println!("{n}");
            }

            best.iter()
                .map(|monkey_best| monkey_best.get(&seq).unwrap_or(&0))
                .sum()
        })
        .max()
        .unwrap()
}

fn generate(start: i64) -> Vec<Foo> {
    let mut result = Vec::with_capacity(2000);
    result.push(Foo {
        secret: start,
        price: start % 10,
        delta: -999999,
    });
    for i in 1..=2000 {
        let prev = result[i - 1].secret;
        let next = fun_name(prev);
        let price = next % 10;
        let delta = price - result[i - 1].price;
        result.push(Foo {
            secret: next,
            price,
            delta,
        });
    }
    result
}

#[derive(Debug, PartialEq, Eq)]
struct Foo {
    secret: i64,
    price: i64,
    delta: i64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            part1(
                "1
10
100
2024
"
            ),
            37327623
        );
    }

    #[test]
    fn test2() {
        let input = "1
2
3
2024
        ";
        assert_eq!(part2(input), 23);
    }
}
