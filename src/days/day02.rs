pub fn part1(input: &str) -> i64 {
    let result = input.split(",").map(|range| {
        let (a, b) = range.split_once("-").expect("unable to split range");
        let start: i64 = a.trim().parse().expect("invalid start id");
        let end: i64 = b.trim().parse().expect("invalid end id");

        (start..=end).filter(|&id| is_double(id)).sum::<i64>()
    });
    result.sum()
}

pub fn part2(input: &str) -> i64 {
    let data = input.split(',').map(|range| {
        let (a, b) = range.split_once('-').expect("unable to split range");
        let start: i64 = a.trim().parse().expect("invalid start id");
        let end: i64 = b.trim().parse().expect("invalid end id");
        let mut result: Vec<i64> = Vec::new();

        for id in start..=end {
            let s = id.to_string();
            let len = s.len();
            let mut is_repeating = false;

            for n in 1..=(len / 2) {
                if len % n == 0 {
                    let repeating_id = &s[0..n];
                    let mut matches_all = true;
                    for i in (n..len).step_by(n) {
                        if &s[i..i + n] != repeating_id {
                            matches_all = false;
                            break;
                        }
                    }
                    if matches_all {
                        is_repeating = true;
                        break;
                    }
                }
            }

            if is_repeating {
                result.push(id);
            }
        }

        result.iter().sum::<i64>()
    });

    data.sum()
}

fn is_double(id: i64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }
    if s.starts_with('0') {
        return false;
    }
    let mid = len / 2;
    s[0..mid] == s[mid..len]
}
