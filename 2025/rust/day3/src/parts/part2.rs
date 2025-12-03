use std::cmp::Ordering;

pub fn part2(input: &str) -> i64 {
    let mut joltvolt: u64 = 0;
    for line in input.lines() {
        let mut lastidx = 0;
        let mut digit: u64 = 0;
        for dig in 0..12 {
            let (mut i, c) = line
                .chars()
                .skip(lastidx)
                .enumerate()
                .filter(|(i, _)| *i < line.len() - 11 + dig - lastidx)
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .unwrap();
            for (j, cc) in line.chars().skip(lastidx).enumerate() {
                if c == cc {
                    i = j;
                    break;
                }
            }
            let x = c.to_digit(10).unwrap() as u64;
            lastidx += i + 1;
            digit = (digit * 10) + x;
        }
        joltvolt += digit;
    }
    joltvolt as i64
}
