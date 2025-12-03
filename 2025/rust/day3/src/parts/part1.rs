pub fn part1(input: &str) -> i32 {
    let mut joltvolt: u32 = 0;
    for line in input.lines() {
        let (mut i, mut z) = (0, 0u32);
        let digits: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        for dig in 0..2 {
            (i, z) = digits
                .iter()
                .skip(i)
                .enumerate()
                .filter(|&(j, _)| j < line.len() - 1 + dig - i)
                .min_by_key(|&(_, x)| 9 - x)
                .map(|(j, x)| (j + i + 1, (z * 10) + x))
                .unwrap();
        }
        joltvolt += z;
    }
    joltvolt as i32
}
