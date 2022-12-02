use aoc::aoc;

#[aoc(2022, 1, 1)]
fn main(input: &str) -> u32 {
    let calories = input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split_whitespace()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    calories
}
