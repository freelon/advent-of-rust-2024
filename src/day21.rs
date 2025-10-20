use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

pub fn day_main() {
    let input = read_to_string("input/day21.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = i64;

fn part1(input: &str) -> RiddleResult {
    let mut p = Precomputed::new(2);
    input
        .lines()
        .map(|line| p.shortest(line) * value(line))
        .sum()
}

fn value(line: &str) -> i64 {
    line[0..3].parse::<i64>().unwrap()
}

fn part2(input: &str) -> RiddleResult {
    let mut p = Precomputed::new(25);
    input
        .lines()
        .map(|line| p.shortest(line) * value(line))
        .sum()
}

struct Precomputed {
    num_sp: HashMap<(char, char), Vec<&'static str>>,
    arrow_sp: HashMap<(char, char), Vec<&'static str>>,
    robot_layers: i64,
    cache: HashMap<(char, char, i64), i64>,
}

impl Precomputed {
    fn new(robot_layers: i64) -> Self {
        let num_sp = num_sp();
        let arrow_sp = arrow_sp();
        Self {
            num_sp,
            arrow_sp,
            robot_layers,
            cache: HashMap::new(),
        }
    }

    fn shortest(&mut self, digit_pad: &str) -> RiddleResult {
        format!("A{digit_pad}")
            .chars()
            .tuple_windows()
            .map(|(a, b)| {
                let x = self
                    .num_sp
                    .get(&(a, b))
                    .unwrap()
                    .clone()
                    .iter()
                    .map(|sp| format!("A{sp}A")) // we add the A in the next layer at the end of each sequence
                    .map(|sp| {
                        let x = if sp.len() > 1 {
                            sp.chars()
                                .tuple_windows()
                                .map(|(c, d)| self.get(c, d, self.robot_layers))
                                .sum::<i64>()
                        } else {
                            1
                        };
                        // println!(" sp {a} to {b} (NUM): {sp} -- {x}");
                        x
                    })
                    .min()
                    .unwrap();
                // println!("{a}{b}: {x}");
                x
            })
            // .inspect(|r| println!("shortest part: {r}"))
            .sum()
    }

    fn get(&mut self, a: char, b: char, n: i64) -> i64 {
        if n == 0 {
            return 1;
        }
        if let Some(result) = self.cache.get(&(a, b, n)) {
            return *result;
        }
        let paths = self.arrow_sp.get(&(a, b)).unwrap().clone();
        let result = paths
            .iter()
            .map(|sp| format!("A{sp}A"))
            .map(|sp| {
                let x = if sp.len() > 1 {
                    sp.chars()
                        .tuple_windows()
                        .map(|(c, d)| self.get(c, d, n - 1))
                        .sum::<i64>()
                } else {
                    1
                };
                // println!(
                //     "{}sp {a} to {b} ({n}): {sp} -- {x}",
                //     " ".repeat(self.robot_layers as usize + 2 - n as usize)
                // );
                x
            })
            .min()
            .unwrap();
        self.cache.insert((a, b, n), result);
        result
    }
}

fn arrow_sp() -> HashMap<(char, char), Vec<&'static str>> {
    let mut starters = HashMap::new();

    starters.insert(('A', '<'), vec!["<v<", "v<<"]);
    starters.insert(('A', '^'), vec!["<"]);
    starters.insert(('A', '>'), vec!["v"]);
    starters.insert(('A', 'v'), vec!["<v", "v<"]);

    starters.insert(('<', '^'), vec![">^"]);
    starters.insert(('<', '>'), vec![">>"]);
    starters.insert(('<', 'v'), vec![">"]);

    starters.insert(('^', '>'), vec![">v", "v>"]);
    starters.insert(('^', 'v'), vec!["v"]);

    starters.insert(('>', 'v'), vec!["<"]);

    let mut result = starters.clone();
    for ((from, to), paths) in starters {
        result.insert((to, from), invert(&paths));
    }

    for c in "A<>v^".chars() {
        result.insert((c, c), vec![""]);
    }

    result
}

fn num_sp() -> HashMap<(char, char), Vec<&'static str>> {
    let mut starters = HashMap::new();

    starters.insert(('A', '0'), vec!["<"]);
    starters.insert(('A', '1'), vec!["<^<", "^<<"]);
    starters.insert(('A', '2'), vec!["<^", "^<"]);
    starters.insert(('A', '3'), vec!["^"]);
    starters.insert(('A', '4'), vec!["<^<^", "<^^<", "^<<^", "^<^<", "^^<<"]);
    starters.insert(('A', '5'), vec!["<^^", "^<^", "^^<"]);
    starters.insert(('A', '6'), vec!["^^"]);
    starters.insert(
        ('A', '7'),
        vec![
            "<^<^^", "<^^<^", "<^^^<", "^<<^^", "^<^<^", "^<^^<", "^^^<<",
        ],
    );
    starters.insert(('A', '8'), vec!["<^^^", "^<^^", "^^<^", "^^^<"]);
    starters.insert(('A', '9'), vec!["^^^"]);

    starters.insert(('0', '1'), vec!["^<"]);
    starters.insert(('0', '2'), vec!["^"]);
    starters.insert(('0', '3'), vec!["^>", ">^"]);
    starters.insert(('0', '4'), vec!["^<^", "^^<"]);
    starters.insert(('0', '5'), vec!["^^"]);
    starters.insert(('0', '6'), vec!["^^>", "^>^"]);
    starters.insert(('0', '7'), vec!["^<^^", "^^<^", "^^^<"]);
    starters.insert(('0', '8'), vec!["^^^"]);
    starters.insert(('0', '9'), vec!["^^^>", "^^>^", "^>^^", ">^^^"]);

    starters.insert(('1', '2'), vec![">"]);
    starters.insert(('1', '3'), vec![">>"]);
    starters.insert(('1', '4'), vec!["^"]);
    starters.insert(('1', '5'), vec!["^>", ">^"]);
    starters.insert(('1', '6'), vec!["^>>", ">^>", ">>^"]);
    starters.insert(('1', '7'), vec!["^^"]);
    starters.insert(('1', '8'), vec!["^^>", "^>^", ">^^"]);
    starters.insert(('1', '9'), vec!["^^>>", "^>^>", "^>>^", ">^>^", ">>^^"]);

    starters.insert(('2', '3'), vec![">"]);
    starters.insert(('2', '4'), vec!["<^", "^<"]);
    starters.insert(('2', '5'), vec!["^"]);
    starters.insert(('2', '6'), vec!["^>", ">^"]);
    starters.insert(('2', '7'), vec!["<^^", "^<^", "^^<"]);
    starters.insert(('2', '8'), vec!["^^"]);
    starters.insert(('2', '9'), vec!["^^>", "^>^", ">^^"]);

    starters.insert(('3', '4'), vec!["<<^", "<^<", "^<<"]);
    starters.insert(('3', '5'), vec!["<^", "^<"]);
    starters.insert(('3', '6'), vec!["^"]);
    starters.insert(
        ('3', '7'),
        vec!["<<^^", "<^<^", "<^^<", "^<^<", "^<<^", "^^<<"],
    );
    starters.insert(('3', '8'), vec!["<^^", "^<^", "^^<"]);
    starters.insert(('3', '9'), vec!["^^"]);

    starters.insert(('4', '5'), vec![">"]);
    starters.insert(('4', '6'), vec![">>"]);
    starters.insert(('4', '7'), vec!["^"]);
    starters.insert(('4', '8'), vec!["^>", ">^"]);
    starters.insert(('4', '9'), vec!["^>>", ">^>", ">>^"]);

    starters.insert(('5', '6'), vec![">"]);
    starters.insert(('5', '7'), vec!["<^", "^<"]);
    starters.insert(('5', '8'), vec!["^"]);
    starters.insert(('5', '9'), vec!["^>", ">^"]);

    starters.insert(('6', '7'), vec!["<<^", "<^<", "^<<"]);
    starters.insert(('6', '8'), vec!["<^", "^<"]);
    starters.insert(('6', '9'), vec!["^"]);

    starters.insert(('7', '8'), vec![">"]);
    starters.insert(('7', '9'), vec![">>"]);

    starters.insert(('8', '9'), vec![">"]);

    let mut result = starters.clone();
    for ((from, to), paths) in starters {
        result.insert((to, from), invert(&paths));
    }

    result
}

fn invert(paths: &Vec<&'static str>) -> Vec<&'static str> {
    paths.iter().map(|path| reverse_path(path)).collect()
}

fn reverse_path(path: &str) -> &'static str {
    path.chars()
        .rev()
        .map(|d| opposite(d))
        .collect::<String>()
        .leak()
}

fn opposite(d: char) -> char {
    match d {
        '<' => '>',
        '^' => 'v',
        '>' => '<',
        'v' => '^',
        _ => panic!("unknown direction {d}"),
    }
}

#[cfg(test)]
mod test {
    use crate::day21::Precomputed;

    use super::{part1, part2};

    const TEST_INPUT: &str = r"029A
980A
179A
456A
379A
";

    #[test]
    fn example1() {
        assert_eq!(part1(TEST_INPUT), 126384);
    }

    #[test]
    fn example1_mini() {
        assert_eq!(part1("029A"), 68 * 29);
    }

    #[test]
    fn shortest_human() {
        let mut precomputed = Precomputed::new(2);
        assert_eq!(precomputed.get('<', '^', 0), 1);
    }

    #[test]
    fn shortest_1st() {
        let mut p = Precomputed::new(2);
        assert_eq!(p.get('A', '^', 1), 2);
    }

    #[test]
    fn shortest_2nd() {
        let mut p = Precomputed::new(2);
        assert_eq!(p.get('A', '^', 2), 8);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
