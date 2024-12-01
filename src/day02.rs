use std::fs::read_to_string;

pub fn day_main() {
    let input = read_to_string("input/day01.txt").unwrap();
    println!("part1: {}", part1(&input));
    // println!("part2: {}", part2(&input));
}

type Int = i32;

fn part1(input: &str) -> Int {
    0
}

// fn part2(input: &str) -> Int {
//     0
// }
