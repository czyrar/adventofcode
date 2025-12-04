#[cfg(test)]
mod test {
    use day4::parts::part1::*;
    use day4::parts::part2::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../../../data/2025/day4/test.in");
        let result = 13;
        assert_eq!(part1(input), result);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../../../data/2025/day4/test.in");
        let result = 43;
        assert_eq!(part2(input), result);
    }
}
