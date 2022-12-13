use std::cmp::Reverse;
use std::mem;

use aoc::aoc;
use itertools::Itertools;
use pest::Parser as _;
use pest_derive::Parser;

#[aoc(2022, 11, 1)]
fn main(input: &str) -> usize {
    let mut monkeys = input.split("\n\n")
        .map(Monkey::parse)
        .collect::<Vec<_>>();

    for _ in 0..20 {
        do_round(&mut monkeys);
    }

    monkeys
        .iter()
        .map(|monkey| monkey.num_inspects)
        .sorted_by_key(|num_inspects| Reverse(*num_inspects))
        .take(2)
        .product()
}

fn do_round(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        let throws = monkeys[i].do_inspections();

        distribute_throws(throws, monkeys);
    }
}

fn distribute_throws(throws: Vec<Throw>, monkeys: &mut [Monkey]) {
    for throw in throws {
        monkeys[throw.target].items.push(throw.item);
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i32>,
    op: Op,
    test: i32,
    on_true: usize,
    on_false: usize,
    num_inspects: usize,
}

impl Monkey {
    fn parse(s: &str) -> Self {
        let mut pairs = MonkeyParser::parse(Rule::monkey, s).unwrap();

        let id = pairs.next().unwrap().as_str().parse::<usize>().unwrap();

        let items = pairs
            .next()
            .unwrap()
            .into_inner()
            .map(|item| item.as_str().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let op = pairs.next().unwrap();
        let op = match op.as_rule() {
            Rule::op_add => {
                let value = op.into_inner().next().unwrap().as_str().parse::<i32>().unwrap();

                Op::Add(value)
            },
            Rule::op_mul => {
                let value = op.into_inner().next().unwrap().as_str().parse::<i32>().unwrap();

                Op::Mul(value)
            },
            Rule::op_square => Op::Square,
            _ => unreachable!(),
        };

        let test = pairs.next().unwrap().as_str().parse::<i32>().unwrap();
        let on_true = pairs.next().unwrap().as_str().parse::<usize>().unwrap();
        let on_false = pairs.next().unwrap().as_str().parse::<usize>().unwrap();

        Self {
            id,
            items,
            op,
            test,
            on_true,
            on_false,
            num_inspects: 0,
        }
    }

    fn do_inspections(&mut self) -> Vec<Throw> {
        let mut throws = Vec::new();

        for mut item in mem::take(&mut self.items) {
            self.num_inspects += 1;
            self.op.apply_to(&mut item);
            item /= 3;

            let target = if item % self.test == 0 {
                self.on_true
            } else {
                self.on_false
            };

            throws.push(Throw {
                item,
                target
            });
        }

        throws
    }
}

struct Throw {
    item: i32,
    target: usize,
}

#[derive(Debug)]
enum Op {
    Add(i32),
    Mul(i32),
    Square,
}

impl Op {
    fn apply_to(&self, lhs: &mut i32) {
        match self {
            Op::Add(rhs) => *lhs += rhs,
            Op::Mul(rhs) => *lhs *= rhs,
            Op::Square => *lhs *= *lhs,
        }
    }
}

#[derive(Parser)]
#[grammar = "monkey.pest"]
struct MonkeyParser;
