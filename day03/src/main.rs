use std::collections::HashSet;

use aoc::aoc;

#[aoc(2022, 3, 1)]
fn main(input: &str) -> u32 {
    input
        .lines()
        .map(|sack| {
            let mid = sack.len() / 2;
            let (comp1, comp2) = sack.split_at(mid);
            let comp1 = comp1.chars().collect::<HashSet<_>>();
            let comp2 = comp2.chars().collect::<HashSet<_>>();
            let mut intersection = comp1.intersection(&comp2);
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
