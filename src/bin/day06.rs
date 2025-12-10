use std::fs;

fn calculate_from_operator_str(operator_str: &str, a: i64, b: i64) -> i64 {
    let final_result = match operator_str {
        "+" => a + b,
        "*" => a * b,
        _ => {
            eprintln!("Error: Unrecognized operator: {}", operator_str);
            return 0;
        }
    };
    final_result
}

fn calculate_numbers(numbers: &Vec<i64>, operator_str: &str) -> i64 {
    let final_result = match operator_str {
        "+" => numbers.iter().sum(),
        "*" => numbers.iter().product(),
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
    let operators = lines.pop().unwrap();
    let mut totals: Vec<i64> = Vec::new();
    for col in 0..lines[0].len() {
        let first_value = lines[0][col].parse::<i64>().unwrap();
        let mut result = first_value;
        for row in 1..lines.len() {
            let value = lines[row][col].parse::<i64>().unwrap();
            result = calculate_from_operator_str(operators[col], result, value);
        }
        totals.push(result);
    }
    totals.iter().sum()
}



fn part_two(input: &str) -> i64 {
    let mut lines: Vec<&str> = input
        .lines()
        .filter(|p| !p.trim().is_empty())
        .collect();
    let operators: Vec<&str> = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .rev()
        .collect();
    let max_len = lines
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);
    let grid:Vec<Vec<char>> = lines
        .iter()
        .map(|&s| {
            let mut row: Vec<char> = s.chars().collect();
            row.resize(max_len, ' ');
            row
        })
        .collect();

    let mut digits_total: Vec<Vec<i64>> = Vec::new();
    let mut current: Vec<i64> = Vec::new();
    
    for col in (0..max_len).rev() {
        
        let mut digit_string = String::new();
        for row in 0..grid.len() {
            let c = grid[row][col];
            if c.is_ascii_digit() {
                digit_string.push(c);
            }
        }

        if digit_string.is_empty() {
            if !current.is_empty() {
                digits_total.push(std::mem::take(&mut current));
            }
        } else if let Ok(num) = digit_string.parse::<i64>() {
            current.push(num);
        }
        if col == 0 && !current.is_empty() {
            digits_total.push(std::mem::take(&mut current));
        }
    }
    digits_total
    .iter()
    .zip(operators.iter())
    .map(|(digits, &op)| calculate_numbers(digits, op))
    .sum()
}
//3261038365331
//8342588849093
fn main() {
    let input = fs::read_to_string("inputs/day06.txt").unwrap_or_default();
    println!("Day06, Part1: {}", part_one(&input));
    println!("Day06, Part2: {}", part_two(&input));
}
