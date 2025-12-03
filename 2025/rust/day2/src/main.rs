mod parts;
use crate::parts::part1::*;
use crate::parts::part2::*;

fn main() {
    let input = include_str!("../../../../data/2025/day2/input.in");
    println!("PART 1: the sum of invalid IDs is {}", part1(input));
    println!("PART 2: the sum of invalid IDs is {}", part2(input));
}
