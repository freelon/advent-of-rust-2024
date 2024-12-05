use itertools::Itertools;
use std::str::FromStr;
use std::{cmp::Ordering, fs::read_to_string};

pub fn day_main() {
    let input = read_to_string("input/day05.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type R = usize;

fn part1(input: &str) -> R {
    let (rules, books) = parse(input);
    books
        .iter()
        .filter(|book| valid_book(book, &rules))
        .map(|book| book[book.len() / 2])
        .sum()
}

fn valid_book(book: &[R], rules: &[(R, R)]) -> bool {
    for (i, a) in book.iter().enumerate() {
        for b in book.iter().skip(i + 1) {
            if rules.contains(&(*b, *a)) {
                return false;
            }
        }
    }
    true
}

fn parse(input: &str) -> (Vec<(R, R)>, Vec<Vec<R>>) {
    let (a, b) = input.split_once("\n\n").unwrap();
    let rules = a
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| (R::from_str(a).unwrap(), R::from_str(b).unwrap()))
        .collect_vec();
    let books = b
        .lines()
        .map(|line| line.split(",").flat_map(R::from_str).collect_vec())
        .collect_vec();

    (rules, books)
}

fn part2(input: &str) -> R {
    let (rules, books) = parse(input);
    books
        .iter()
        .filter(|book| !valid_book(book, &rules))
        .map(|book| fix(book, &rules))
        .map(|book| book[book.len() / 2])
        .sum()
}

fn fix(book: &[R], rules: &[(R, R)]) -> Vec<R> {
    let mut b = book.iter().copied().collect_vec();
    b.sort_unstable_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            Ordering::Less
        } else if rules.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    b
}

#[cfg(test)]
mod test {
    use super::{part1, part2, valid_book};

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
        assert!(valid_book(&vec![97, 5, 13], &vec![(97, 13)]));
        assert!(!valid_book(&vec![13, 5, 97], &vec![(97, 13)]));
        assert_eq!(part1(TEST_INPUT), 143);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 123);
    }
}
