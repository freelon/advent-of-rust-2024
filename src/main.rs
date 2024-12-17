use advent_of_rust_2024::*;
use itertools::Itertools;
use std::time::Instant;
use std::{collections::HashMap, env::args};

fn main() {
    let mains: HashMap<u8, fn() -> ()> = HashMap::from_iter([
        (1, day01::day_main as fn()),
        (2, day02::day_main),
        (3, day03::day_main),
        (4, day04::day_main),
        (5, day05::day_main),
        (6, day06::day_main),
        (7, day07::day_main),
        (8, day08::day_main),
        (9, day09::day_main),
        (10, day10::day_main),
        (11, day11::day_main),
        (12, day12::day_main),
        (13, day13::day_main),
        (14, day14::day_main),
        (15, day15::day_main),
        (16, day16::day_main),
        (17, day17::day_main),
        // PLACEHOLDER
    ]);
    let day: Option<u8> = args().nth(1).and_then(|a| a.parse().ok());
    let Some(day) = day else {
        mains
            .iter()
            .sorted_by_key(|entry| entry.0)
            .for_each(|(d, f)| {
                run(*d, f);
            });
        return;
    };

    let Some(f) = mains.get(&day) else {
        println!("No implementation for day {day}.");
        println!(
            "Pick one of {}",
            mains.keys().map(|k| k.to_string()).join(", ")
        );
        return;
    };

    run(day, f);
}

fn run(d: u8, f: &fn()) {
    println!("Day {d}:");
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    println!("{COLOR}{ITALIC}Took {duration:?}{RESET_FORMATTING}");
}

const COLOR: &str = "\x1b[38;5;247m";
const ITALIC: &str = "\x1b[3m";
const RESET_FORMATTING: &str = "\x1b[0m";
