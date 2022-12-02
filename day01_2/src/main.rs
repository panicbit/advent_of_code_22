use std::cmp::Reverse;

use aoc::aoc;
use itertools::Itertools;

#[aoc(2022, 1, 2)]
fn main(input: &str) -> u32 {
    let calories = input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split_whitespace()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_by_key(|calories| Reverse(*calories))
        .take(3)
        .sum();

    calories
}
