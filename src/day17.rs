use std::{fmt::Write, fs::read_to_string, io::BufRead, ops::BitXor};

use itertools::Itertools;

pub fn day_main() {
    let input = read_to_string("input/day17.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = String;

fn part1(input: &str) -> RiddleResult {
    let (first, second) = input.split_once("\n\n").unwrap();
    dbg!(first);
    dbg!(second);
    let mut registers = first.lines().map(|line| line.split_once(": ").unwrap().1);

    let mut a = registers.next().unwrap().parse::<i64>().unwrap();
    let mut b = registers.next().unwrap().parse::<i64>().unwrap();
    let mut c = registers.next().unwrap().parse::<i64>().unwrap();

    let program = second
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|v| v.parse::<i64>().unwrap())
        .collect_vec();

    let mut ip = 0;
    let mut output = String::new();

    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[(ip + 1) % program.len()];
        // println!("a: {a}, b: {b}, c: {c}, op={opcode} {operand} (ip: {ip})");
        match opcode {
            0 => a = a / (2i64.pow(val(a, b, c, operand) as u32)),
            1 => b = b.bitxor(operand),
            2 => b = val(a, b, c, operand) % 8,
            3 => {
                if a == 0 {
                } else {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => b = b.bitxor(c),
            5 => output
                .write_fmt(format_args!("{},", val(a, b, c, operand) % 8))
                .unwrap(),
            6 => b = a / (2i64.pow(val(a, b, c, operand) as u32)),
            7 => c = a / (2i64.pow(val(a, b, c, operand) as u32)),
            _ => panic!("illegal op code {}", opcode),
        }
        ip += 2;
        // ip %= program.len();
        // wait();
    }
    if output.ends_with(",") {
        output.remove(output.len() - 1);
    }
    output
}

fn wait() {
    let stdin = std::io::stdin();
    let _line1 = stdin.lock().lines().next().unwrap().unwrap();
}

fn val(a: i64, b: i64, c: i64, operand: i64) -> i64 {
    match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("illegal operand {}", operand),
    }
}

fn part2(_input: &str) -> RiddleResult {
    String::new()
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
    ";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT.trim()), "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT.trim()), "".to_string());
    }
}
