use std::cmp::Reverse;
use std::mem;

use aoc::aoc;
use itertools::Itertools;
use num::integer::lcm;
use pest::Parser as _;
use pest_derive::Parser;

#[aoc(2022, 11, 2)]
fn main(input: &str) -> usize {
    let mut monkeys = input.split("\n\n")
        .map(Monkey::parse)
        .collect::<Vec<_>>();

    let lcm = monkeys.iter()
        .map(|monkey| monkey.test)
        .reduce(lcm)
        .unwrap();

    for _ in 0..10_000 {
        do_round(&mut monkeys, lcm);
    }

    monkeys
        .iter()
        .map(|monkey| monkey.num_inspects)
        .sorted_by_key(|num_inspects| Reverse(*num_inspects))
        .take(2)
        .product()
}

fn do_round(monkeys: &mut [Monkey], lcm: i64) {
    for i in 0..monkeys.len() {
        let throws = monkeys[i].do_inspections(lcm);

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
    items: Vec<i64>,
    op: Op,
    test: i64,
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
            .map(|item| item.as_str().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let op = pairs.next().unwrap();
        let op = match op.as_rule() {
            Rule::op_add => {
                let value = op.into_inner().next().unwrap().as_str().parse::<i64>().unwrap();

                Op::Add(value)
            },
            Rule::op_mul => {
                let value = op.into_inner().next().unwrap().as_str().parse::<i64>().unwrap();

                Op::Mul(value)
            },
            Rule::op_square => Op::Square,
            _ => unreachable!(),
        };

        let test = pairs.next().unwrap().as_str().parse::<i64>().unwrap();
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

    fn do_inspections(&mut self, lcm: i64) -> Vec<Throw> {
        let mut throws = Vec::new();

        for mut item in mem::take(&mut self.items) {
            self.num_inspects += 1;
            self.op.apply_to(&mut item);
            item %= lcm;

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
    item: i64,
    target: usize,
}

#[derive(Debug)]
enum Op {
    Add(i64),
    Mul(i64),
    Square,
}

impl Op {
    fn apply_to(&self, lhs: &mut i64) {
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
