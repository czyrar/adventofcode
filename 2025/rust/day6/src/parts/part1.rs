pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let operations: Vec<char> = lines
        .next_back()
        .unwrap()
        .chars()
        .filter(|&x| x != ' ')
        .collect();
    let mut numbers = lines
        .into_iter()
        .map(|x| {
            x.trim()
                .split_whitespace()
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .into_iter();
    let mut results = numbers.next().unwrap();
    for vec in numbers {
        for (i, (x, op)) in vec
            .into_iter()
            .zip(operations.to_owned().into_iter())
            .enumerate()
        {
            match op {
                '*' => results[i] *= x,
                '+' => results[i] += x,
                _ => unreachable!(),
            }
        }
    }
    results.into_iter().sum()
}
