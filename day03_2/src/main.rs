use std::collections::btree_set::Intersection;
use std::collections::HashSet;

use aoc::aoc;
use itertools::Itertools;

#[aoc(2022, 3, 2)]
fn main(input: &str) -> u32 {
    input
        .lines()
        .map(|sack| sack.chars().collect::<HashSet<_>>())
        .tuples()
        .map(|(sack1, sack2, sack3)| {
            let sack12_intersection = sack1.intersection(&sack2).copied().collect::<HashSet<_>>();
            let mut intersection = sack12_intersection.intersection(&sack3);
            let common_item = *intersection.next().unwrap();

            priority(common_item)
        })
        .sum()
}

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}
