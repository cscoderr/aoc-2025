use std::fs;

use aoc_2025::days::day01;

fn main() {
    let day = 1;

    match day {
        1 => {
            let input = fs::read_to_string("inputs/day01.txt").unwrap_or_default();
            let day01_part1 = day01::part1(&input);
            println!("Day 1, Part 1: {}", day01_part1);
        }
        _ => panic!("Day not implemented"),
    }
}
