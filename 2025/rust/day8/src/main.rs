mod parts;
use crate::parts::part1::*;
use crate::parts::part2::*;

fn main() {
    let input = include_str!("../../../../data/2025/day8/input.in");
    println!(
        "PART 1: multiplication of the sizes gives {}",
        part1(input, 1000)
    );
    println!(
        "PART 1: multiplication of the X coords give {}",
        part2(input)
    );
}
