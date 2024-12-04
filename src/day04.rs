use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn day_main() {
    let input = read_to_string("input/day04.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    let mut puzzle = HashMap::<(usize, usize), char>::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, chr)| {
            puzzle.insert((x, y), chr);
        })
    });

    diags(&puzzle)
    // 2023 wrong
}

fn diags(m: &HashMap<(usize, usize), char>) -> usize {
    let mut count = 0;
    let xmax = m.keys().map(|k| k.0).max().unwrap();
    let ymax = m.keys().map(|k| k.1).max().unwrap();
    println!("{xmax},{ymax}");
    let S = "XMAS".chars().collect_vec();

    for sx in 0..=xmax {
        for sy in 0..=ymax {
            'foo: for (a, b) in [
                (1, 1),
                (-1, 1),
                (1, -1),
                (-1, -1),
                (0, 1),
                (1, 0),
                (0, -1),
                (-1, 0),
            ] {
                for (i, c) in S.iter().enumerate() {
                    let i = i as i32;
                    let p = (sx as i32 + i * a, sy as i32 + i * b);
                    if p.0 < 0 || p.1 < 0 || p.0 > xmax as i32 || p.1 > ymax as i32 {
                        continue 'foo;
                    }

                    if m[&(p.0 as usize, p.1 as usize)] != *c {
                        continue 'foo;
                    }
                }
                println!("found from {sx},{sy} ({a},{b})");
                count += 1;
            }
        }
    }

    count
}

fn part2(_input: &str) -> RiddleResult {
    0
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
