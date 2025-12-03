mod parts;
use crate::parts::part1::*;
use crate::parts::part2::*;

fn main() {
    let input = include_str!("../../../../data/2025/day3/input.in");
    println!("PART 1: the total output joltage is {}", part1(input));
    println!("PART 2: the total output joltage is {}", part2(input));
}
