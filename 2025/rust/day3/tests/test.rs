#[cfg(test)]
mod test {
    use day3::parts::part1::*;
    use day3::parts::part2::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../data/test.dat");
        let result = 357;
        assert_eq!(part1(input), result);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../data/test.dat");
        let result = 3121910778619i64;
        assert_eq!(part2(input), result);
    }
}
