#[cfg(test)]
mod test {
    use day5::parts::part1::*;
    use day5::parts::part2::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../../../data/2025/day5/test.in");
        let result = 3;
        assert_eq!(part1(input), result);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../../../data/2025/day5/test.in");
        let result = 14;
        assert_eq!(part2(input), result);
    }
}
