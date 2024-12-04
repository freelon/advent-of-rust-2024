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
    let puzzle = parse_map(input);
    let mut count = 0;
    let x_max = puzzle.keys().map(|k| k.0).max().unwrap();
    let y_max = puzzle.keys().map(|k| k.1).max().unwrap();

    for sx in 0..=x_max {
        for sy in 0..=y_max {
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
                for (i, c) in "XMAS".chars().enumerate() {
                    let i = i as i32;
                    let p = (sx as i32 + i * a, sy as i32 + i * b);
                    if p.0 < 0 || p.1 < 0 || p.0 > x_max as i32 || p.1 > y_max as i32 {
                        continue 'foo;
                    }

                    if puzzle[&(p.0 as usize, p.1 as usize)] != c {
                        continue 'foo;
                    }
                }
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &str) -> RiddleResult {
    let puzzle = parse_map(input);
    let mut count = 0;
    let x_max = puzzle.keys().map(|k| k.0).max().unwrap();
    let y_max = puzzle.keys().map(|k| k.1).max().unwrap();
    for sx in 1..x_max {
        for sy in 1..y_max {
            let first = [
                puzzle[&(sx - 1, sy - 1)],
                puzzle[&(sx, sy)],
                puzzle[&(sx + 1, sy + 1)],
            ]
            .into_iter()
            .join("");
            let second = [
                puzzle[&(sx - 1, sy + 1)],
                puzzle[&(sx, sy)],
                puzzle[&(sx + 1, sy - 1)],
            ]
            .into_iter()
            .join("");
            if (first == "MAS" || first == "SAM") && (second == "SAM" || second == "MAS") {
                count += 1;
            }
        }
    }

    count
}

fn parse_map(input: &str) -> HashMap<(usize, usize), char> {
    let mut puzzle = HashMap::<(usize, usize), char>::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, chr)| {
            puzzle.insert((x, y), chr);
        })
    });
    puzzle
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
        assert_eq!(part2(TEST_INPUT), 9);
    }
}
