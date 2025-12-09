mod parts;
use crate::parts::part1::*;
use crate::parts::part2::*;

fn main() {
    let input = include_str!("../../../../data/2025/day7/input.in");
    println!("PART 1: it has splitted {} times", part1(input));
    println!("PART 2: it has created {} universes", part2(input));
}
