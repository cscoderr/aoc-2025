pub fn part1(input: &str) -> i64 {
    let datas = input.lines();
    let mut pos: i64 = 50;
    const MAX: i64 = 99;
    let mut result = Vec::new();
    for data in datas {
        let (direction, num_str) = data.split_at(1);
        let value: i64 = num_str.parse().expect("Invalid number");

        match direction {
            "R" => pos = (pos + value) % (MAX + 1),
            "L" => {
                pos = (pos - value) % (MAX + 1);
                if pos < 0 {
                    pos += MAX + 1;
                }
            }
            _ => panic!("Invalid direction"),
        }

        result.push(pos);
    }
    let count_zero = result.iter().filter(|&&v| v == 0).count();
    count_zero as i64
}

pub fn part2(input: &str) -> i64 {
    let datas = input.lines();
    let mut pos: i64 = 50;
    const MAX: i64 = 99;
    let mut result = Vec::new();
    let mut point_result = Vec::new();
    for data in datas {
        let (direction, num_str) = data.split_at(1);
        let value: i64 = num_str.parse().expect("Invalid number");

        match direction {
            "R" => pos = (pos + value) % (MAX + 1),
            "L" => {
                pos = (pos - value) % (MAX + 1);
                if pos < 0 {
                    pos += MAX + 1;
                }
            }
            _ => panic!("Invalid direction"),
        }

        result.push(pos);
    }
    let count_result = result.iter().filter(|&&v| v == 0).count();
    let count_point_result = result.iter().filter(|&&v| v == 0).count();
    (count_result + count_point_result) as i64
}
