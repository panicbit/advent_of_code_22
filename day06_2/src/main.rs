use std::collections::HashSet;

use aoc::aoc;

#[aoc(2022, 6, 2)]
fn main(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .position(|chars| chars.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap() + 14
}
