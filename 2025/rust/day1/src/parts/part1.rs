pub fn part1(input: &str) -> i16 {
    let mut password: i16 = 0;
    let mut position: i16 = 50;
    for line in input.lines() {
        let number: i16 = line[1..].parse().unwrap();
        if line.starts_with("R") {
            position = (position + number) % 100;
        } else {
            position = (position - number).rem_euclid(100);
        }
        if position == 0 {
            password += 1;
        }
    }
    return password;
}
