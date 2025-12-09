#[cfg(test)]
mod test {
    use day7::parts::part1::*;
    use day7::parts::part2::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../../../data/2025/day7/test.in");
        let result = 21;
        assert_eq!(part1(input), result);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../../../data/2025/day7/test.in");
        let result = 40;
        assert_eq!(part2(input), result);
    }
}
