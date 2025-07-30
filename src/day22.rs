use std::{collections::HashSet, fs::read_to_string, ops::BitXor};

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

    let all_quadruples = sequences(&deltas);
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

            deltas
                .iter()
                .map(|monkey| find_first(seq, monkey))
                .flat_map(|first| first.map(|f| f.3.price).or(Some(0)))
                .sum()
        })
        .max()
        .unwrap()
}

fn sequences(deltas: &Vec<Vec<Foo>>) -> HashSet<(i64, i64, i64, i64)> {
    let mut all_quadruples = HashSet::new();
    for monkey in deltas {
        monkey
            .iter()
            .skip(1)
            .tuple_windows()
            .for_each(|(a, b, c, d)| {
                all_quadruples.insert((a.delta, b.delta, c.delta, d.delta));
            });
    }
    all_quadruples
}

fn find_first(seq: (i64, i64, i64, i64), monkey: &Vec<Foo>) -> Option<(&Foo, &Foo, &Foo, &Foo)> {
    monkey.iter().tuple_windows().find(|(a, b, c, d)| {
        a.delta == seq.0 && b.delta == seq.1 && c.delta == seq.2 && d.delta == seq.3
    })
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

    #[test]
    fn finds_first() {
        let monkey = generate(123);
        let f = find_first((-1, -1, 0, 2), &monkey);
        assert_eq!(
            &Foo {
                secret: 12683156,
                price: 6,
                delta: 2,
            },
            f.unwrap().3
        );
    }
}
