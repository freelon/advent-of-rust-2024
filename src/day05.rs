use itertools::Itertools;
use std::{cmp::Ordering, fs::read_to_string};

pub fn day_main() {
    let input = read_to_string("input/day05.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    let (rules, books) = parse(input);
    books.iter().filter(|book| {
        valid_book(book, &rules)
    })
    .map(|book| book[book.len() / 2].parse::<RiddleResult>().unwrap())
    .sum()
}

fn valid_book(book: &[&str], rules: &[(&str, &str)]) -> bool {
    for (i, a) in book.iter().enumerate() {
        for b in book.iter().skip(i + 1) {
            if rules.contains(&(b, a)) {
                return false;
            }
        }
    }
    true
}

fn parse(input: &str) -> (Vec<(&str, &str)>, Vec<Vec<&str>>) {
    let (a, b) = input.split_once("\n\n").unwrap();
    let rules = a.lines().map(|line| line.split_once("|").unwrap()).collect_vec();
    let books = b.lines().map(|line| line.split(",").collect_vec()).collect_vec();

    (rules, books)
}

fn part2(input: &str) -> RiddleResult {
    let (rules, books) = parse(input);
    books.iter().filter(|book| {
        !valid_book(book, &rules)
    })
    .map(|book| fix(book, &rules))
    .map(|book| book[book.len() / 2].parse::<RiddleResult>().unwrap())
    .sum()
}

fn  fix<'a>(book: &'a[&str], rules: &[(&str, &str)]) -> Vec<&'a str> {
    let mut b = book.iter().copied().collect_vec();
    b.sort_unstable_by(|a, b| {
        if rules.contains(&(a, b)) {
            Ordering::Less
        } else if rules.contains(&(b, a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    b
}

#[cfg(test)]
mod test {
    use crate::day05::valid_book;

    use super::{part1, part2};

    const TEST_INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test1() {
        assert!(valid_book(&vec!["97", "5", "13"], &vec![("97", "13")]));
        assert!(!valid_book(&vec!["13", "5", "97"], &vec![("97", "13")]));
        assert_eq!(part1(TEST_INPUT), 143);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 123);
    }
}
