use std::collections::HashSet;
use std::ops;

use aoc::aoc;
use itertools::Itertools;

#[aoc(2022, 9, 2)]
fn main(input: &str) -> usize {
    let motions = input.lines().map(Motion::parse);
    let mut state = State::new();

    for motion in motions {
        state.apply_motion(&motion);
    }

    state.tail_visited.len()
}

struct State {
    knots: Vec<Vec2>,
    tail_visited: HashSet<Vec2>,
}

impl State {
    fn new() -> Self {
        let mut this = Self {
            knots: vec![Vec2::default(); 10],
            tail_visited: HashSet::new(),
        };

        this.tail_visited.insert(Vec2::default());

        this
    }

    fn apply_motion(&mut self, motion: &Motion) {
        for _ in 0..motion.amount {
            self.apply_direction(&motion.direction);
        }
    }

    fn apply_direction(&mut self, direction: &Direction) {
        self.knots[0] += direction.vector();

        for (head, tail) in (0..self.knots.len()).tuple_windows() {

            let vector = self.knots[head] - self.knots[tail];

            if vector.x.abs() <= 1 && vector.y.abs() <= 1 {
                return;
            }

            self.knots[tail] += vector.signum();
        }

        self.tail_visited.insert(*self.knots.last().unwrap());
    }
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn signum(&self) -> Vec2 {
        Vec2 {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    amount: usize,
}

impl Motion {
    fn parse(s: &str) -> Self {
        let (direction, amount) = s.split_once(' ').unwrap();

        Self {
            direction: Direction::parse(direction),
            amount: amount.parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }

    fn vector(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2 { x: 0, y: -1 },
            Direction::Down => Vec2 { x: 0, y: 1 },
            Direction::Left => Vec2 { x: -1, y: 0 } ,
            Direction::Right => Vec2 { x: 1, y: 0 },
        }
    }
}
