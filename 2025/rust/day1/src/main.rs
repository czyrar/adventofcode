mod parts;
use crate::parts::part1::*;
use crate::parts::part2::*;

fn main() {
    let input = include_str!("../data/input.dat");
    println!("PART 1: The password is {}", part1(input));
    println!("PART 2: The password is {}", part2(input));
}
