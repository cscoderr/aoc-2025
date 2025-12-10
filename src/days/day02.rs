pub fn part1(input: &str) -> i64 {
    let result = input.split(",").filter(|s| !s.is_empty()).map(|range| {
        let (a, b) = range.split_once("-").expect("unable to split range");
        let start: i64 = a.trim().parse().expect("invalid start id");
        let end: i64 = b.trim().parse().expect("invalid end id");

        (start..=end).filter(|&id| is_double(id)).sum::<i64>()
    });
    result.sum()
}

pub fn part2(input: &str) -> i64 {
    
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

// pub fn part2(input: &str) {}
