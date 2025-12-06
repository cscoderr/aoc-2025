use chrono::{Datelike, Utc};
use std::fs;

use aoc_2025::days::{day01, day02, day03};

fn main() {
    let day = Utc::now().day() - 3;

    match day {
        1 => {
            let input = fs::read_to_string("inputs/day01.txt").unwrap_or_default();
            println!("Day {day:0>2}, Part1: {}", day01::part1(&input));
            println!("Day {day:0>2}, Part2: {}", day01::part2(&input));
        }
        2 => {
            let input = fs::read_to_string("inputs/day02.txt").unwrap_or_default();
            println!("Day {day:0>2}, Part1: {}", day02::part1(&input));
            println!("Day {day:0>2}, Part2: {}", day02::part2(&input));
        }
        3 => {
            let input = fs::read_to_string("inputs/day03.txt").unwrap_or_default();
            println!("Day {day:0>2}, Part1: {}", day03::part1(&input));
            println!("Day {day:0>2}, Part2: {}", day03::part2(&input));
        }
        _ => panic!("Day not implemented"),
    }
}
