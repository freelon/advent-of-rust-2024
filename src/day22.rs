use std::{fs::read_to_string, ops::BitXor};

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
        secret = prune(mix(secret * 64, secret));
        secret = prune(mix(secret / 32, secret));
        secret = prune(mix(secret * 2048, secret))
    }
    secret
}

fn mix(value: i64, secret: i64) -> i64 {
    value.bitxor(secret)
}

fn prune(value: i64) -> i64 {
    value % 16777216
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
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
