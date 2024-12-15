use std::{
    collections::HashSet,
    fs::read_to_string,
    // io::{self, BufRead},
};

use itertools::Itertools;

use crate::utils::grid::Grid;

pub fn day_main() {
    let input = read_to_string("input/day14.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    solve_part1(input, 100, 101, 103)
}

fn solve_part1(input: &str, rounds: i64, width: i64, height: i64) -> RiddleResult {
    let mut robots = parse(input);
    for _ in 0..rounds {
        robots.iter_mut().for_each(|((px, py), (vx, vy))| {
            *px = (*px + width + *vx) % width;
            *py = (*py + height + *vy) % height;
        });
    }
    let upper_left = robots
        .iter()
        .filter(|&&((px, py), _)| px < width / 2 && py < height / 2)
        .count();
    let lower_left = robots
        .iter()
        .filter(|&&((px, py), _)| px < width / 2 && py > height / 2)
        .count();
    let upper_right = robots
        .iter()
        .filter(|&&((px, py), _)| px > width / 2 && py < height / 2)
        .count();
    let lower_right = robots
        .iter()
        .filter(|&&((px, py), _)| px > width / 2 && py > height / 2)
        .count();

    upper_left * lower_left * upper_right * lower_right
}

fn parse(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input
        .lines()
        .map(|line| {
            line.strip_prefix("p=")
                .unwrap()
                .split(" v=")
                .map(|s| {
                    s.split(",")
                        .map(|d| d.parse::<i64>().unwrap())
                        .collect_tuple::<(i64, i64)>()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

#[allow(unused)]
fn part2(input: &str) -> RiddleResult {
    let width = 101;
    let height = 103;
    let mut robots = parse(input);
    let mut seen = HashSet::new();
    for second in 0.. {
        if seen.contains(&robots) {
            panic!("Loop after {second} rounds, but no christmas tree!");
        }
        let mut grid: Grid<u8> = Grid::from_default(101, 103);
        seen.insert(robots.clone());
        robots.iter_mut().for_each(|((px, py), (vx, vy))| {
            *px = (*px + width + *vx) % width;
            *py = (*py + height + *vy) % height;
            grid[(*px, *py)] += 1;
            if let Some(v) = grid.get_mut((*px + 1, *py)) {
                *v += 1
            }
            if let Some(v) = grid.get_mut((*px - 1, *py)) {
                *v += 1
            }
            if let Some(v) = grid.get_mut((*px, *py + 1)) {
                *v += 1
            }
            if let Some(v) = grid.get_mut((*px, *py - 1)) {
                *v += 1
            }
        });

        if robots.iter().filter(|(s, _)| grid[*s] > 1).count() > robots.len() * 70 / 100 {
            // printr(&robots, width, height);
            // println!("after {} seconds. Press enter to continue or type 'merry christmas' if you can spot a tree!", second + 1); //+1 because we look at it after they have changed
            return second + 1;
            // let stdin = io::stdin();
            // let line = stdin.lock().lines().next().unwrap().unwrap();
            // if line.as_str().eq_ignore_ascii_case("merry christmas") {
            // return second + 1;
            // }
        }
    }
    unreachable!()
}

#[allow(unused)]
type Robot = ((i64, i64), (i64, i64));

#[allow(unused)]
fn printr(robots: &[Robot], width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            let c = robots
                .iter()
                .filter(|&&((px, py), _)| px == x && py == y)
                .count();
            if c == 0 {
                print!(".");
            } else {
                print!("{c}");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use crate::day14::solve_part1;

    const TEST_INPUT: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test1() {
        assert_eq!(solve_part1(TEST_INPUT, 100, 11, 7), 12);
    }
}
