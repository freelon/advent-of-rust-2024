use std::{collections::HashMap, fs::read_to_string, hash::Hash};

use itertools::Itertools;

pub fn day_main() {
    let input = read_to_string("input/day24.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = u64;

fn part1(input: &str) -> RiddleResult {
    let (a, b) = input.split_once("\n\n").unwrap();
    let program = b
        .lines()
        .map(|l| {
            let (lhs, rhs) = l.split_once(" -> ").unwrap();
            (rhs, lhs)
        })
        .collect_vec();
    let program = HashMap::from_iter(program.into_iter());
    let start_values = HashMap::from_iter(a.lines().map(|l| {
        let (x, y) = l.split_once(": ").unwrap();
        let b = match y {
            "0" => false,
            "1" => true,
            _ => panic!("bad values {y}"),
        };
        (x, b)
    }));
    let binary_result = program
        .keys()
        .filter(|k| k.starts_with("z"))
        .sorted_unstable()
        .map(|z| compute(&program, &start_values, z))
        .map(|b| match b {
            true => "1",
            false => "0",
        })
        .rev()
        .join("");

    u64::from_str_radix(&binary_result, 2).unwrap()
}

fn compute(program: &HashMap<&str, &str>, start_values: &HashMap<&str, bool>, start: &str) -> bool {
    if start_values.contains_key(start) {
        return start_values[start];
    }

    let v = program[start].split(" ").collect_vec();
    let lhs = compute(program, start_values, v[0]);
    let rhs = compute(program, start_values, v[2]);
    match v[1] {
        "AND" => lhs & rhs,
        "OR" => lhs | rhs,
        "XOR" => lhs ^ rhs,
        _ => panic!("bad operator {}", v[1]),
    }
}

fn part2(_input: &str) -> RiddleResult {
    0
}

/// Trait that provides a `collect_map` method for iterators over key-value tuples.
pub trait CollectMap<K, V>: Iterator<Item = (K, V)>
where
    K: Eq + Hash,
{
    /// Collects the iterator into a `HashMap`.
    fn collect_map(self) -> HashMap<K, V>;
}

// Blanket implementation for any compatible iterator
impl<I, K, V> CollectMap<K, V> for I
where
    I: Iterator<Item = (K, V)>,
    K: Eq + Hash,
{
    fn collect_map(self) -> HashMap<K, V> {
        self.collect()
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 2024);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
