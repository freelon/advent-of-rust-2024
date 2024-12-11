use std::{collections::HashMap, fs::read_to_string};

pub fn day_main() {
    let input = read_to_string("input/day11.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    solve(input, 25)
}

fn solve(input: &str, blinks: i32) -> usize {
    let mut stones: HashMap<usize, usize> =
        input.split(" ").map(|v| (v.parse().unwrap(), 1)).collect();
    let mut next = HashMap::new();
    for _ in 0..blinks {
        for (k, v) in stones {
            if k == 0 {
                next.entry(1).and_modify(|count| *count += v).or_insert(v);
            } else if (k.ilog10() + 1) % 2 == 0 {
                let l = (k.ilog10() + 1) / 2;
                let z: usize = 10usize.pow(l as u32);
                let a = k / z;
                let b = k % z;
                next.entry(a).and_modify(|count| *count += v).or_insert(v);
                next.entry(b).and_modify(|count| *count += v).or_insert(v);
            } else {
                let k_new = k * 2024;
                next.entry(k_new)
                    .and_modify(|count| *count += v)
                    .or_insert(v);
            }
        }
        stones = next;
        next = HashMap::new();
    }
    stones.values().to_owned().sum()
}

fn part2(input: &str) -> RiddleResult {
    solve(input, 75)
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn test1() {
        assert_eq!(part1("125 17"), 55312);
    }
}
