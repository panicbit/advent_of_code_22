use std::collections::BTreeMap;

use aoc::aoc;
use utils::{re, StrExt};

#[aoc(2022, 5, 2)]
fn main(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let layers = stacks.lines().map(parse_layer);
    let mut stacks = merge_layers(layers);
    let instructions = instructions.lines().map(Instruction::parse);

    for instruction in instructions {
        let stack = stacks.get_mut(&instruction.from).unwrap();
        let cargos = stack.drain(stack.len() - instruction.amount..).collect::<Vec<_>>();

        stacks.get_mut(&instruction.to).unwrap().extend(cargos);
    }

    let message =stacks.values()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();
    
    message
}

fn parse_layer(s: &str) -> Vec<Option<char>> {
    let re = re!(r#"[\[ ](.)[\] ] ?"#);

    let mut layer = Vec::new();

    for cap in re.captures_iter(s) {
        let cargo = cap.get(1).unwrap().as_str().char();
        let cargo = (cargo.is_alphabetic()).then_some(cargo);

        layer.push(cargo);
    }

    layer
}

fn merge_layers(layers: impl Iterator<Item = Vec<Option<char>>>) -> BTreeMap<usize, Vec<char>> {
    let mut stacks = BTreeMap::<usize, Vec<char>>::new();

    for layer in layers {
        for (i, cargo) in layer.iter().copied().enumerate() {
            let i = i + 1;
            let Some(cargo) = cargo else { continue };

            stacks.entry(i).or_default().push(cargo);
        }
    }

    for stack in &mut stacks.values_mut() {
        stack.reverse();
    }

    stacks
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let cap = re!(r"move (\d+) from (\d+) to (\d+)").captures(s).unwrap();

        Self {
            amount: cap[1].usize(),
            from: cap[2].usize(),
            to: cap[3].usize(),
        }
    }
}
