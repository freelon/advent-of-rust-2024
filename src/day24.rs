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
        .collect_map();
    let start_values = a
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(": ").unwrap();
            let b = match y {
                "0" => false,
                "1" => true,
                _ => panic!("bad values {y}"),
            };
            (x, b)
        })
        .collect_map();
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

const SWAPS: [(&'static str, &'static str); 4] = [
    ("rkf", "z09"),
    ("jgb", "z20"),
    ("vcg", "z24"),
    ("rvc", "rrs"),
];

fn swap(s: &str) -> &str {
    for (a, b) in &SWAPS {
        if *a == s {
            return b;
        }
        if *b == s {
            return a;
        }
    }
    s
}

fn part2(input: &str) -> String {
    let (_, b) = input.split_once("\n\n").unwrap();
    let program = b
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(def, name)| (def, swap(name)))
        .collect_map();

    if program["y00 XOR x00"] != "z00" {
        panic!("first bit is bad already");
    }
    let Some(mut c_prev) = find(&program, "y00", "AND", "x00") else {
        panic!("first carrier missing");
    };
    for i in 1..=44 {
        // println!();
        // println!("## {i:02}");
        let x_i = format!("x{:02}", i);
        let y_i = format!("y{:02}", i);
        let op = "XOR";
        let Some(xor_i) = find(&program, x_i.as_str(), op, y_i.as_str()) else {
            println!("xor_{:02} missing", i);
            break;
        };
        // println!("xor_i = {xor_i}");
        let Some(z_i) = find(&program, &xor_i, "XOR", &c_prev) else {
            println!("z_{:02} missing", i);
            break;
        };
        if format!("z{i:02}") != z_i {
            println!("z_i is bad: {z_i}");
            break;
        }
        let Some(and_inputs_i) = find(&program, &x_i, "AND", &y_i) else {
            println!("and_inputs_{:02} missing", i);
            break;
        };
        // println!("and_inputs_i = {and_inputs_i}");
        let Some(and_all_i) = find(&program, &c_prev, "AND", &xor_i) else {
            println!("and of last carrier with xor_{:02} missing", i);
            break;
        };
        // println!("and_all_i = {and_all_i}");
        let Some(c_i) = find(&program, &and_all_i, "OR", &and_inputs_i) else {
            println!("c_{:02} missing", i);
            break;
        };
        // println!("c_i = {c_i}");
        c_prev = c_i;
    }
    SWAPS
        .iter()
        .map(|(a, b)| [a, b])
        .flatten()
        .sorted()
        .join(",")
}

fn find(program: &HashMap<&str, &str>, a: &str, op: &str, b: &str) -> Option<String> {
    program
        .get(format!("{a} {op} {b}").as_str())
        .or_else(|| program.get(format!("{b} {op} {a}").as_str()))
        .map(|s| s.to_string())
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
    use super::part1;

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
}
