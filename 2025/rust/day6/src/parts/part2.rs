pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let operations: Vec<_> = lines.next_back().unwrap().chars().rev().collect();
    let numbers: Vec<_> = lines
        .into_iter()
        .rev()
        .map(|x| x.chars().rev().filter(|&y| y != '\n').collect::<Vec<_>>())
        .collect();
    let rows = numbers.len();
    let cols = numbers[0].len();
    let trans: Vec<_> = (0..cols)
        .map(|col| (0..rows).map(|row| numbers[row][col]).collect::<Vec<_>>())
        .collect();

    let mut result: u64 = 0;
    let mut op = 0;
    let mut res = Vec::new();
    for number in trans {
        let mut num: u64 = 0;
        let mut mult = 1;
        for digit in number {
            if let Some(d) = digit.to_digit(10) {
                num += (d * mult) as u64;
                mult *= 10;
            }
        }
        if num > 0 {
            res.push(num);
        }
        if !operations[op].is_whitespace() {
            result += res
                .to_owned()
                .into_iter()
                .reduce(|acc, x| match operations[op] {
                    '*' => acc * x,
                    '+' => acc + x,
                    _ => unreachable!(),
                })
                .unwrap();
            res.clear();
        }
        op += 1;
    }
    result
}
