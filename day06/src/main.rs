use std::collections::HashSet;

use aoc::aoc;
use itertools::Itertools;

#[aoc(2022, 6, 1)]
fn main(input: &str) -> usize {
    input
        .chars()
        .tuple_windows()
        .position(|(a, b, c, d)| HashSet::from([a, b, c, d]).len() == 4)
        .unwrap() + 4
}
