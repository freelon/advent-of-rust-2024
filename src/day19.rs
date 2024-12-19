use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn day_main() {
    let input = read_to_string("input/day19.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    let (a, b) = input.split_once("\n\n").unwrap();
    let towels: HashSet<&str> = a.split(", ").collect();
    b.lines().filter(|line| possible(line, &towels)).count()
}

fn possible(line: &str, towels: &HashSet<&str>) -> bool {
    if line.is_empty() {
        return true;
    }
    for t in towels.iter() {
        if let Some(suffix) = line.strip_prefix(t) {
            if possible(suffix, towels) {
                return true;
            }
        }
    }
    false
}

fn countp<'a>(
    line: &'a str,
    towels: &HashSet<&str>,
    solved: &mut HashMap<&'a str, usize>,
) -> usize {
    if line.is_empty() {
        return 1;
    }
    if let Some(count) = solved.get(line) {
        return *count;
    }
    let mut summed = 0;
    for t in towels.iter() {
        if let Some(suffix) = line.strip_prefix(t) {
            summed += countp(suffix, towels, solved);
        }
    }
    solved.insert(line, summed);
    summed
}
fn part2(input: &str) -> RiddleResult {
    let (a, b) = input.split_once("\n\n").unwrap();
    let towels: HashSet<&str> = a.split(", ").collect();
    b.lines()
        .map(|line| countp(line, &towels, &mut HashMap::new()))
        .sum()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::day19::possible;

    use super::{part1, part2};

    const TEST_INPUT: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 6);
    }

    #[test]
    fn solver() {
        let set = HashSet::from_iter(["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
        assert!(possible("bwurrg", &set));
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 16);
    }
}
