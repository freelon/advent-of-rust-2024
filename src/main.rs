use std::env::args;

use advent_of_rust_2024::day01;

fn main() {
    let day: Option<u8> = args().nth(1).and_then(|a| a.parse().ok());
    match day {
        Some(1) => day01::day_main(),
        _ => println!("hi"),
    }
}
