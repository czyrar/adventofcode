#[cfg(test)]
mod test {
    use day1::parts::part1::*;
    use day1::parts::part2::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../../../data/2025/day1/test.in");
        let result = 3;
        assert_eq!(part1(input), result);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../../../data/2025/day1/test.in");
        let result = 6;
        assert_eq!(part2(input), result);
    }
}
