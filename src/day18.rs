use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use itertools::Itertools;

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

fn solve1(input: &str, n: usize, coord_max: i32) -> usize {
    let points = parse(input);
    sp(&points[..n], coord_max).unwrap()
}

fn sp(points: &[(i32, i32)], coord_max: i32) -> Option<usize> {
    let pointss: HashSet<&(i32, i32)> = HashSet::from_iter(points);
    let mut visited = HashMap::new();
    let mut queue = VecDeque::from_iter([(0, 0, 0)]);
    while let Some((x, y, c)) = queue.pop_front() {
        if x == coord_max && y == coord_max {
            return Some(c);
        }

        if visited.contains_key(&(x, y)) {
            continue;
        }
        visited.insert((x, y), c);

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (a, b) = (x + dx, y + dy);
            if (0..=coord_max).contains(&a)
                && (0..=coord_max).contains(&b)
                && !pointss.contains(&(a, b))
            {
                queue.push_back((a, b, c + 1));
            }
        }
    }
    None
}

fn parse(input: &str) -> Vec<(i32, i32)> {
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

fn solve2(input: &str, fixed: usize, max_coord: i32) -> String {
    let points = parse(input);
    for i in fixed..points.len() {
        if let None = sp(&points[..=i], max_coord) {
            return format!("{},{}", points[i].0, points[i].1);
        }
    }
    unreachable!()
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
