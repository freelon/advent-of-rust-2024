use std::{collections::VecDeque, fs::read_to_string};

use itertools::Itertools;

use crate::utils::grid::Grid;

pub fn day_main() {
    let input = read_to_string("input/day18.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    solve1(input, 1024, 70)
}

fn solve1(input: &str, n: usize, coord_max: i64) -> usize {
    let points = parse(input);
    sp(&points[..n], coord_max).unwrap()
}

fn sp(points: &[(i64, i64)], coord_max: i64) -> Option<usize> {
    let mut pcheck = Grid::from_default(coord_max + 1, coord_max + 1);
    for p in points {
        pcheck.set(*p, true);
    }
    let mut visited = Grid::from_default(coord_max + 1, coord_max + 1);
    let mut queue = VecDeque::from_iter([(0, 0, 0)]);
    while let Some((x, y, c)) = queue.pop_front() {
        if x == coord_max && y == coord_max {
            return Some(c);
        }

        if visited.get((x, y)) == Some(&true) {
            continue;
        }
        visited.set((x, y), true);

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (a, b) = (x + dx, y + dy);
            if (0..=coord_max).contains(&a)
                && (0..=coord_max).contains(&b)
                && pcheck.get((a, b)) == Some(&false)
            {
                queue.push_back((a, b, c + 1));
            }
        }
    }
    None
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn part2(input: &str) -> String {
    solve2(input, 1024, 70)
}

fn solve2(input: &str, fixed: usize, max_coord: i64) -> String {
    let points = parse(input);

    // we want the first point with which there is no shortest path
    let mut i = fixed;
    let mut jump = (points.len() - fixed) / 2;
    #[cfg(debug_assertions)]
    let mut loops = 0;
    loop {
        #[cfg(debug_assertions)]
        {
            loops += 1;
        }
        let l = sp(&points[..i - 1], max_coord);
        let r = sp(&points[..i], max_coord);
        if l.is_some() && r.is_none() {
            #[cfg(debug_assertions)]
            {
                let iters_needed = i - fixed;
                dbg!(loops, iters_needed);
            }
            return format!("{},{}", points[i - 1].0, points[i - 1].1);
        }
        if l.is_some() {
            i += jump;
        } else {
            i -= jump;
        }
        jump /= 2;
        if jump == 0 {
            jump += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day18::{solve1, solve2};

    const TEST_INPUT: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test1() {
        assert_eq!(solve1(TEST_INPUT, 12, 6), 22);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(TEST_INPUT, 12, 6), "6,1");
    }
}
