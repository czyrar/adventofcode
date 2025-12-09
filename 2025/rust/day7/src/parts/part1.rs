pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let (start, _) = lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .find(|&(_, x)| x == 'S')
        .unwrap();
    let mut splitted = 0;
    let mut fall: Vec<usize> = vec![0; lines.next().unwrap().chars().collect::<Vec<_>>().len()];
    fall[start] = 1;
    while let Some(line) = lines.next() {
        for (i, c) in line.chars().enumerate() {
            if c == '^' && fall[i] == 1 {
                fall[i + 1] = 1;
                fall[i - 1] = 1;
                fall[i] = 0;
                splitted += 1
            }
        }
    }
    splitted
}
