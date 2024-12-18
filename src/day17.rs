use std::{fs::read_to_string, ops::BitXor};

use itertools::Itertools;

pub fn day_main() {
    let input = read_to_string("input/day17.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = String;

fn part1(input: &str) -> RiddleResult {
    let (a, b, c, program) = parse_input(input);

    run(&program, a, b, c)
        .into_iter()
        .map(|it| it.to_string())
        .join(",")
}

fn run(program: &Vec<i64>, mut a: i64, mut b: i64, mut c: i64) -> Vec<i64> {
    let mut ip = 0;
    let mut output = vec![];

    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[(ip + 1) % program.len()];
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
            5 => output.push(val(a, b, c, operand) % 8),
            6 => b = a / (2i64.pow(val(a, b, c, operand) as u32)),
            7 => c = a / (2i64.pow(val(a, b, c, operand) as u32)),
            _ => panic!("illegal op code {}", opcode),
        }
        ip += 2;
    }
    output
}

fn parse_input(input: &str) -> (i64, i64, i64, Vec<i64>) {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut registers = first.lines().map(|line| line.split_once(": ").unwrap().1);

    let a = registers.next().unwrap().parse::<i64>().unwrap();
    let b = registers.next().unwrap().parse::<i64>().unwrap();
    let c = registers.next().unwrap().parse::<i64>().unwrap();

    let program = second
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|v| v.parse::<i64>().unwrap())
        .collect_vec();
    (a, b, c, program)
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

fn part2(input: &str) -> RiddleResult {
    let (_, _, _, program) = parse_input(input);
    solve(&program, program.len() - 1, 0).unwrap().to_string()
}

fn solve(program: &Vec<i64>, index: usize, a: i64) -> Option<i64> {
    // if index == 0 {
    //     return if &run(program, a, 0, 0) == program {
    //         Some(a)
    //     } else {
    //         None
    //     };
    // }
    for i in 0..8 {
        let next_a = a * 8 + i;
        let o = run(program, next_a, 0, 0);
        if o[0] == program[index] {
            if index == 0 {
                return Some(next_a);
            } else {
                let m = solve(program, index - 1, next_a);
                if m.is_some() {
                    return m;
                }
            }
        }
    }
    None
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
        assert_eq!(
            part2(
                "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
                .trim()
            ),
            "117440".to_string()
        );
    }
}
