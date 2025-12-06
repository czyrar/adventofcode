use regex::Regex;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> usize {
    let re_ranges = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    let mut ranges: Vec<RangeInclusive<i64>> = Vec::new();
    let mut start = 0;

    for (i, line) in input.lines().enumerate() {
        if let Some(capture) = re_ranges.captures(line) {
            let i0: i64 = capture.get(1).unwrap().as_str().parse().unwrap();
            let i1: i64 = capture.get(2).unwrap().as_str().parse().unwrap();
            ranges.push(i0..=i1);
            continue;
        }
        start = i + 1;
        break;
    }

    input
        .lines()
        .skip(start)
        .map(|x| x.parse::<i64>().unwrap())
        .filter(|x| {
            for range in &ranges {
                if range.contains(x) {
                    return true;
                }
            }
            false
        })
        .count()
}
