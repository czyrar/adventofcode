#[cfg(test)]
mod test {
    use day2::parts::part1::*;
    use day2::parts::part2::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../../data/day2/test.in");
        let result = 1227775554;
        assert_eq!(part1(input), result);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../../data/day2/test.in");
        let result = 4174379265;
        assert_eq!(part2(input), result);
    }
}
