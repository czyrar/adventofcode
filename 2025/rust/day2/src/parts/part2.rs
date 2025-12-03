use onig::Regex;

pub fn part2(input: &str) -> i64 {
    let re = Regex::new(r"^(\d+)\1+$").unwrap();
    let mut sum_invalid: i64 = 0;
    for part in input.split(",") {
        let range = part.trim().split("-").collect::<Vec<&str>>();
        let start = range[0].parse::<i64>().unwrap();
        let end = range[1].parse::<i64>().unwrap();
        for i in start..=end {
            if re.is_match(i.to_string().as_str()) {
                sum_invalid += i;
            }
        }
    }
    return sum_invalid;
}
