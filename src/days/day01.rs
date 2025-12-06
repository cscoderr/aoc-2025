const MAX: i64 = 100;

pub fn part1(input: &str) -> usize {
    let mut pos: i64 = 50;
    input
        .lines()
        .map(|input| {
            let (direction, num_str) = input.split_at(1);
            let value: i64 = num_str.parse().expect("Invalid number");
            pos = get_pos(value, pos, direction);
            pos
        })
        .filter(|&d| d == 0)
        .count()
}
pub fn part2(input: &str) -> i64 {
    let mut pos: i64 = 50;
    let mut total_zero_encounters: i64 = 0;

    for input in input.lines() {
        let (direction, num_str) = input.split_at(1);
        let value: i64 = num_str.parse().expect("Invalid number");

        let distance_to_zero = match direction {
            "R" => {
                if pos == 0 {
                    MAX
                } else {
                    MAX - pos
                }
            }

            "L" => {
                if pos == 0 {
                    MAX
                } else {
                    pos
                }
            }

            _ => unreachable!(),
        };
        let encounter = if value < distance_to_zero {
            0
        } else {
            1 + (value - distance_to_zero) / MAX
        };

        total_zero_encounters += encounter;

        pos = get_pos(value, pos, direction);
    }

    total_zero_encounters
}

fn get_pos(value: i64, pos: i64, direction: &str) -> i64 {
    match direction {
        "R" => (pos + value) % MAX,
        "L" => {
            let mut temp_pos = (pos - value) % MAX;
            if temp_pos < 0 {
                temp_pos += MAX;
            }
            temp_pos
        }
        _ => unreachable!(),
    }
}
