use std::fs;

fn calculate_from_operator_str(operator_str: &str, a: i64, b: i64) -> i64 {
    let final_result = match operator_str {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        _ => {
            eprintln!("Error: Unrecognized operator: {}", operator_str);
            return 0;
        }
    };
    final_result
}
fn part_one(input: &str) -> i64 {
    let mut lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let symbols = lines.pop().unwrap();
    let mut totals: Vec<i64> = Vec::new();
    for col in 0..lines[0].len() {
        let first_value = lines[0][col].parse::<i64>().unwrap();
        let mut result = first_value;
        for row in 1..lines.len() {
            let value = lines[row][col].parse::<i64>().unwrap();
            result = calculate_from_operator_str(symbols[col], result, value);
        }
        totals.push(result);
    }
    totals.iter().sum()
}

fn part_two(input: &str) -> i64 {
    0
}
//3261038365331
fn main() {
    let input = fs::read_to_string("inputs/day06.txt").unwrap_or_default();
    println!("Day06, Part1: {}", part_one(&input));
    println!("Day06, Part2: {}", part_two(&input));
}
