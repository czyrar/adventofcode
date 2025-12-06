use regex::Regex;

pub fn part2(input: &str) -> usize {
    let re_ranges = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    let mut ranges: Vec<[usize; 2]> = input
        .lines()
        .filter(|x| re_ranges.is_match(x))
        .map(|x| {
            let capture = re_ranges.captures(x).unwrap();
            let i0: usize = capture.get(1).unwrap().as_str().parse().unwrap();
            let i1: usize = capture.get(2).unwrap().as_str().parse().unwrap();
            [i0, i1]
        })
        .collect();
    ranges.sort_by_key(|x| x[0]);
    let mut ranges_union: Vec<[usize; 2]> = Vec::new();
    for range in ranges {
        if let Some(prev) = ranges_union.last_mut() {
            if range[0] <= prev[1] + 1 {
                prev[1] = prev[1].max(range[1]);
                continue;
            }
        }
        ranges_union.push(range);
    }
    ranges_union
        .into_iter()
        .fold(0, |acc, x| acc + x[1] - x[0] + 1)
}
