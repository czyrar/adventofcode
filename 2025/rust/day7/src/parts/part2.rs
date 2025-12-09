fn explore(lines: &Vec<Vec<char>>, memo: &mut Vec<Vec<isize>>, row: usize, column: usize) -> usize {
    if row == lines.len() {
        return 1;
    }
    // Already computed
    if memo[row][column] >= 0 {
        return memo[row][column] as usize;
    }
    let mut splits = 0;
    if lines[row][column] == '^' {
        splits += explore(lines, memo, row + 1, column - 1);
        splits += explore(lines, memo, row + 1, column + 1);
    } else {
        splits += explore(lines, memo, row + 1, column);
    }
    memo[row][column] = splits as isize;
    splits
}

pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let (start, _) = lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .find(|&(_, x)| x == 'S')
        .unwrap();
    let length = lines.next().unwrap().len();
    let lines: Vec<_> = lines.map(|x| x.chars().collect::<Vec<_>>()).collect();
    let mut memo = vec![vec![-1; length]; lines.len()];
    explore(&lines, &mut memo, 0, start)
}
