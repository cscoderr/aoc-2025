use std::fs;

pub fn part_one(input: &str) -> i64 {
    let inputs: Vec<&str> = input.lines().collect();
    let mut result: Vec<i64> = Vec::new();
    for input in inputs {
        let joltage_ratings: Vec<i64> = input
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid digit in input") as i64)
            .collect();
        let mut total_joltage = 0;
        for i in 0..joltage_ratings.len() {
            let a = joltage_ratings[i];
            for j in (i + 1)..joltage_ratings.len() {
                let b = joltage_ratings[j];
                let current_joltage = a * 10 + b;
                if current_joltage > total_joltage {
                    total_joltage = current_joltage;
                }
            }
        }
        result.push(total_joltage);
    }
    result.iter().sum()
}

pub fn part_two(input: &str) -> i64 {
    let output = input.lines().map(|line| {
        let joltage_ratings: Vec<char> = line.chars().collect();
        let mut result: Vec<char> = Vec::new();
        let mut start_index = 0;
        for k in 0..12 {
            let unselected_digit = 12 - 1 - k;
            let end_index = joltage_ratings.len() - unselected_digit;
            let mut total_joltage = '0';
            let mut current_index: usize = start_index;
            for i in start_index..end_index {
                let current_joltage = joltage_ratings[i];

                if current_joltage > total_joltage {
                    total_joltage = current_joltage;
                    current_index = i;
                }
            }
            result.push(total_joltage);
            start_index = current_index + 1
        }

        result.iter().collect::<String>()
    });
    output.filter_map(|s| s.parse::<i64>().ok()).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").unwrap_or_default();
    println!("Day03, Part1: {}", part_one(&input));
    println!("Day03, Part2: {}", part_two(&input));
}
