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
    ]);
    let day: Option<u8> = args().nth(1).and_then(|a| a.parse().ok());
    let Some(day) = day else {
        mains
            .iter()
            .sorted_by_key(|entry| entry.0)
            .for_each(|(d, f)| {
                println!("Day {d}:");
                let start = Instant::now();
                f();
                let duration = start.elapsed();
                println!(
                    "{color}{italic}Took {duration:?}{reset_formatting}",
                    color = "\x1b[38;5;247m",
                    italic = "\x1b[3m",
                    reset_formatting = "\x1b[0m"
                );
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

    println!("Day {day}:");
    f();
}
