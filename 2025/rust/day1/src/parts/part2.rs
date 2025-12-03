pub fn part2(input: &str) -> i16 {
    let mut password: i16 = 0;
    let mut position: i16 = 50;
    for line in input.lines() {
        let number: i16 = line[1..].parse().unwrap();
        if line.starts_with("R") {
            position += number;
            password += position / 100;
            position = position % 100;
        } else {
            if position == 0 {
                password -= 1;
            }
            position -= number + 1;
            password += position.div_euclid(100).abs();
            position = (position + 1).rem_euclid(100);
        };
    }
    return password;
}
