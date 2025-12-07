use std::fs;

fn part_one(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let neighbors = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != '@' {
                continue;
            }
            let mut roll_count = 0;
            for (nr, nc) in neighbors {
                let r = row as isize + nr;
                let c = col as isize + nc;
                if r >= 0 && r < rows as isize && c >= 0 && c < cols as isize {
                    if grid[r as usize][c as usize] == '@' {
                        roll_count += 1;
                    }
                }
            }
            if roll_count < 4 {
                count += 1;
            }
        }
    }
    count
}

fn part_two(input: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let neighbors = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;

    loop {
        let mut data_to_remove: Vec<(usize, usize)> = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != '@' {
                    continue;
                }

                let mut roll_count = 0;

                for (dr, dc) in neighbors {
                    let r = row as isize + dr;
                    let c = col as isize + dc;

                    if r >= 0 && r < rows as isize && c >= 0 && c < cols as isize {
                        if grid[r as usize][c as usize] == '@' {
                            roll_count += 1;
                        }
                    }
                }

                if roll_count < 4 {
                    data_to_remove.push((row, col));
                }
            }
        }
        if data_to_remove.is_empty() {
            break;
        }
        for (r, c) in &data_to_remove {
            grid[*r][*c] = '.';
            count += 1;
        }
    }
    count
}

fn main() {
    let input = fs::read_to_string("inputs/day04.txt").unwrap_or_default();
    println!("Day04, Part1: {}", part_one(&input));
    println!("Day04, Part2: {}", part_two(&input));
}
