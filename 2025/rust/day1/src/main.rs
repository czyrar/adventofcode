mod parts;
use crate::parts::part1::*;
use crate::parts::part2::*;

fn main() {
    let input = include_str!("../../../../data/2025/day1/input.in");
    println!("PART 1: The password is {}", part1(input));
    println!("PART 2: The password is {}", part2(input));
}
