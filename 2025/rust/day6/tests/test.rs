#[cfg(test)]
mod test {
    use day6::parts::part1::*;
    use day6::parts::part2::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../../../data/2025/day6/test.in");
        let result = 4277556;
        assert_eq!(part1(input), result);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../../../data/2025/day6/test.in");
        let result = 3263827;
        assert_eq!(part2(input), result);
    }
}
