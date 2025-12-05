use chrono::{Datelike, Utc};
use std::fs;

use aoc_2025::days::{day01, day02};

fn main() {
    let day = Utc::now().day() - 1;

    match day {
        1 => {
            let input = fs::read_to_string("inputs/day01.txt").unwrap_or_default();
            println!("Day 1, Part 1: {}", day01::part1(&input));
            println!("Day 1, Part 2: {}", day01::part2(&input));
        }
        2 => {
            let input = fs::read_to_string("inputs/day02.txt").unwrap_or_default();
            println!("Day 2, Part 1: {}", day02::part1(&input));
            println!("Day 2, Part 1: {}", day02::part2(&input));
        }
        _ => panic!("Day not implemented"),
    }
}
