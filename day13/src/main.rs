use std::cmp::Ordering;

use aoc::aoc;
use itertools::{Itertools, EitherOrBoth};
use pest::iterators::Pair;
use pest_derive::Parser;
use pest::Parser as _;

#[aoc(2022, 13, 1)]
fn main(input: &str) -> usize {
    input.split("\n\n")
        .enumerate()
        .filter(|(_, data)| {
            let (left, right) = data.split_once('\n').unwrap();
            let left = parse(left);
            let right = parse(right);

            left.compare(&right).is_lt()
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>()
}

fn parse(input: &str) -> Expr {
    let mut pairs = Parser::parse(Rule::expr, input).unwrap();
    let pair = pairs.next().unwrap();

    parse_expr(pair)
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    assert_eq!(pair.as_rule(), Rule::expr);

    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::list => Expr::List(parse_list(pair)),
        Rule::expr => parse_expr(pair),
        Rule::number => Expr::Number(parse_number(pair)),
    }
}

fn parse_number(pair: Pair<Rule>) -> u32 {
    assert_eq!(pair.as_rule(), Rule::number);

    pair.as_str().parse::<u32>().unwrap()
}

fn parse_list(pair: Pair<Rule>) -> Vec<Expr> {
    assert_eq!(pair.as_rule(), Rule::list);

    pair.into_inner().map(parse_expr).collect()
}

#[derive(Debug, Clone)]
enum Expr {
    List(Vec<Expr>),
    Number(u32),
}

impl Expr {
    fn compare(&self, right: &Expr) -> Ordering {
        match (self, right) {
            (Expr::Number(left), Expr::Number(right)) => left.cmp(&right),
            (Expr::List(left), Expr::List(right)) => {
                for either in left.iter().zip_longest(right) {
                    match either {
                        EitherOrBoth::Both(left, right) => match left.compare(right) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Equal => continue,
                        },
                        EitherOrBoth::Left(_) => return Ordering::Greater,
                        EitherOrBoth::Right(_) => return Ordering::Less,
                    }
                }

                Ordering::Equal
            },
            (left @ Expr::List(_), Expr::Number(right)) => left.compare(&Self::number_to_list(*right)),
            (Expr::Number(left), right @Expr::List(_)) => Self::number_to_list(*left).compare(right),
        }
    }

    fn number_to_list(number: u32) -> Expr {
        Expr::List(vec![Expr::Number(number)])
    }
}

#[derive(Parser)]

#[grammar = "signal.pest"]
struct Parser;
