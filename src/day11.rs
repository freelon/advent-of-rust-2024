use std::{collections::HashMap, fs::read_to_string};

pub fn day_main() {
    let input = read_to_string("input/day11.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    solve(input, 25)
}

fn solve(input: &str, blinks: i32) -> usize {
    let mut stones: HashMap<String, usize> = input.split(" ").map(|v| (v.to_string(), 1)).collect();
    let mut next = HashMap::new();
    for _ in 0..blinks {
        for (k, v) in stones {
            if k.as_str() == "0" {
                next.entry("1".to_string())
                    .and_modify(|count| *count += v)
                    .or_insert(v);
            } else if k.len() % 2 == 0 {
                let (a, b) = k.split_at(k.len() / 2);
                next.entry(a.to_string())
                    .and_modify(|count| *count += v)
                    .or_insert(v);
                let x = b.parse::<usize>().unwrap().to_string();
                next.entry(x).and_modify(|count| *count += v).or_insert(v);
            } else {
                let d: usize = k.parse().unwrap();
                let k_new = d * 2024;
                next.entry(k_new.to_string())
                    .and_modify(|count| *count += v)
                    .or_insert(v);
            }
        }
        stones = next;
        next = HashMap::new();
    }
    stones.values().to_owned().sum()
}

fn part2(input: &str) -> RiddleResult {
    solve(input, 75)
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"";

    #[test]
    fn test1() {
        assert_eq!(part1("125 17"), 55312);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
