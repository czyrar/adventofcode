pub fn part2(input: &str) -> i64 {
    let mut joltvolt: u64 = 0;
    for line in input.lines() {
        let (mut i, mut z) = (0, 0u64);
        for dig in 0..12 {
            (i, z) = line
                .chars()
                .skip(i)
                .map(|x| x.to_digit(10).unwrap() as u64)
                .enumerate()
                .filter(|&(j, _)| j < line.len() - 11 + dig - i)
                .min_by_key(|(_, x)| 9 - x)
                .map(|(j, x)| (j + i + 1, (z * 10) + x))
                .unwrap();
        }
        joltvolt += z;
    }
    joltvolt as i64
}
