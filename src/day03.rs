use std::fs::read_to_string;

pub fn day_main() {
    let input = read_to_string("input/day03.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = i64;

fn part1(input: &str) -> RiddleResult {
    let r = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    r.captures_iter(input)
        .map(|c| {
            let a: i64 = c[1].parse().unwrap();
            let b: i64 = c[2].parse().unwrap();
            a * b
        })
        .sum()
}

fn part2(input: &str) -> RiddleResult {
    let r = regex::Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    r.captures_iter(input)
        // (enabled, result): initially enabled; sum up the result over all enabled multiplications
        .fold((true, 0), |(enabled, result), c| {
            let s = c.get(0).unwrap().as_str();

            match s {
                "do()" => (true, result),
                "don't()" => (false, result),
                _ => {
                    let a: i64 = c[1].parse().unwrap();
                    let b: i64 = c[2].parse().unwrap();
                    (enabled, result + if enabled { a * b } else { 0 })
                }
            }
        })
        .1
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 161);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT2), 48);
    }
}
