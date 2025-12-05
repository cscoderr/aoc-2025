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
    let data = input.split(",").map(|range| {
        let (a, b) = range.split_once("-").expect("unable to split range");
        let start: i64 = a.trim().parse().expect("invalid start id");
        let end: i64 = b.trim().parse().expect("invalid end id");
        let mut result: Vec<i64> = Vec::new();

        for id in start..=end {
            let s = id.to_string();
            let mut index: usize = 1;
            let mut next_index = (index + 1).min(s.len());
            let mut checker = "";
            loop {
                if s[0..index] != s[index..next_index] {
                    index = (index + 1).min(s.len());
                    next_index = (index + 1).min(s.len());
                } else {
                    checker = &s[0..next_index];
                    index = next_index.min(s.len());
                    next_index = (index + 1).min(s.len());
                }
                if s[0..index] == *checker {
                    result.push(s.parse().unwrap());
                    break;
                }
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

// pub fn part2(input: &str) {}
