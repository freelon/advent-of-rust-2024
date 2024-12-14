use std::fs::read_to_string;

use itertools::Itertools;

pub fn day_main() {
    let input = read_to_string("input/day14.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input)); // 50112000 too low
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    solve(input, 100, 101, 103)
}

fn solve(input: &str, rounds: i64, width: i64, height: i64) -> RiddleResult {
    let mut robots: Vec<((i64, i64), (i64, i64))> = input
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
        .collect_vec();
    for _ in 0..rounds {
        robots.iter_mut().for_each(|((px, py), (vx, vy))| {
            *px = (*px + width + *vx) % width;
            *py = (*py + height + *vy) % height;
        });
    }
    // for y in 0..height {
    //     for x in 0..width {
    //         let c = robots
    //             .iter()
    //             .filter(|&&((px, py), _)| px == x && py == y)
    //             .count();
    //         if c == 0 {
    //             print!(".");
    //         } else {
    //             print!("{c}");
    //         }
    //     }
    //     println!();
    // }
    robots
        .iter()
        .filter(|&&((px, py), _)| px < width / 2 && py < height / 2)
        .count()
        * robots
            .iter()
            .filter(|&&((px, py), _)| px < width / 2 && py > height / 2)
            .count()
        * robots
            .iter()
            .filter(|&&((px, py), _)| px > width / 2 && py < height / 2)
            .count()
        * robots
            .iter()
            .filter(|&&((px, py), _)| px > width / 2 && py > height / 2)
            .count()
}

fn part2(_input: &str) -> RiddleResult {
    0
}

#[cfg(test)]
mod test {
    use crate::day14::solve;

    use super::part2;

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
        assert_eq!(solve(TEST_INPUT, 100, 11, 7), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
