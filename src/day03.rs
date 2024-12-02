use std::fs::read_to_string;

pub fn day_main() {
    let input = read_to_string("input/day03.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    0
}

fn part2(_input: &str) -> RiddleResult {
    0
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 0);
    }
    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
