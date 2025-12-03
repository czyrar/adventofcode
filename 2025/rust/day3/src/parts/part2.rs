const LENGTH: usize = 12;

pub fn part2(input: &str) -> i64 {
    let mut joltvolt: u64 = 0;
    for line in input.lines() {
        let (mut i, mut z) = (0, 0u64);
        let digits: Vec<u64> = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        for dig in 0..LENGTH {
            (i, z) = digits
                .iter()
                .enumerate()
                .filter(|(j, _)| (i..=(line.len() - LENGTH + dig)).contains(j))
                .min_by_key(|&(_, x)| 9 - x)
                .map(|(j, x)| (j + 1, (z * 10) + x))
                .unwrap();
        }
        joltvolt += z;
    }
    joltvolt as i64
}
