#[cfg(test)]
mod test {
    use day8::parts::part1::*;
    use day8::parts::part2::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../../../data/2025/day8/test.in");
        let result = 40;
        assert_eq!(part1(input, 10), result);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../../../data/2025/day8/test.in");
        let result = 25272;
        assert_eq!(part2(input), result);
    }
}
