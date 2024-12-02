use std::env::args;

use advent_of_rust_2024::*;

fn main() {
    let day: Option<u8> = args().nth(1).and_then(|a| a.parse().ok());
    match day {
        Some(1) => day01::day_main(),
        Some(2) => day02::day_main(),
        Some(3) => day03::day_main(),
        _ => println!("Please select a day to run"),
    }
}
