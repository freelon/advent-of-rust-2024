use std::fs::read_to_string;

use itertools::Itertools;

pub fn day_main() {
    let input = read_to_string("input/day02.txt").unwrap();
    println!(" part1: {}", part1(&input));
    println!(" part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .filter(|report: &Vec<i32>| is_safe(report))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .filter(|report: &Vec<i32>| {
            (0..report.len()).any(|del| {
                let without = report
                    .iter()
                    .enumerate()
                    .filter_map(|(i, e)| if del != i { Some(*e) } else { None })
                    .collect_vec();
                is_safe(&without)
            })
        })
        .count()
}

fn is_safe(report: &[i32]) -> bool {
    let pairs = report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect_vec();
    let steepest_slope = pairs.iter().map(|x| x.abs()).max().unwrap();
    let lowest_slope = pairs.iter().map(|x| x.abs()).min().unwrap();
    let same_sign = pairs.iter().map(|x| x.signum()).unique().count() == 1;

    steepest_slope <= 3 && lowest_slope >= 1 && same_sign
}

// fn part2(input: &str) -> Int {
//     0
// }
#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(2, super::part1(input));
    }
}
