use std::{cmp::max, collections::HashSet, fs};

fn part_one(input: &str) -> i64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() < 2 {
        return 0;
    }
    let ranges = parts[0];
    let available_ingredients = parts[1];
    let ingredients: HashSet<i64> = available_ingredients
        .lines()
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect();

    let ranges: Vec<(i64, i64)> = ranges
        .lines()
        .filter_map(|line| {
            line.split_once('-').and_then(|(a, b)| {
                let start = a.trim().parse::<i64>().ok()?;
                let end = b.trim().parse::<i64>().ok()?;
                Some((start, end))
            })
        })
        .collect();
    let mut count = 0;
    for ingredient_id in ingredients.iter() {
        let mut is_covered = false;

        for (start, end) in &ranges {
            if ingredient_id >= start && ingredient_id <= end {
                is_covered = true;
                break;
            }
        }

        if is_covered {
            count += 1;
        }
    }

    count
}

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return vec![];
    }
    ranges.sort_by_key(|r| r.0);

    let mut merged = vec![ranges[0]];

    for (start, end) in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if start <= last.1 {
            last.1 = max(last.1, end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}

fn part_two(input: &str) -> i64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() < 2 {
        return 0;
    }
    let ranges = parts[0];

    let ranges: Vec<(i64, i64)> = ranges
        .lines()
        .filter_map(|line| {
            line.split_once('-').and_then(|(a, b)| {
                let start = a.trim().parse::<i64>().ok()?;
                let end = b.trim().parse::<i64>().ok()?;
                Some((start, end))
            })
        })
        .collect();
    let merged = merge_ranges(ranges);
    let mut result = Vec::new();
    for (start, end) in merged {
        let n = end - start + 1;
        result.push(n);
    }

    result.iter().sum()
}
// Day05, Part1: 789
// Day05, Part2: 343329651880509
fn main() {
    let input = fs::read_to_string("inputs/day05.txt").unwrap_or_default();
    println!("Day05, Part1: {}", part_one(&input));
    println!("Day05, Part2: {}", part_two(&input));
}
