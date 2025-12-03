use std::fs;

use aoc_2025::days::day01;

fn main() {
    let day = 1;

    match day {
        1 => {
            let input = fs::read_to_string("inputs/day01.txt").unwrap_or_default();
            println!("Day 1, Part 1: {}", day01::part1(&input));
            println!("Day 1, Part 2: {}", day01::part2(&input));
        }
        _ => panic!("Day not implemented"),
    }
}
