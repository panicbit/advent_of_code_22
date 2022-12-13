use std::collections::HashMap;
use std::iter;

use aoc::aoc;
use take_until::TakeUntilExt;

#[aoc(2022, 8, 2)]
fn main(input: &str) -> usize {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, ch)| ((x, y), ch)))
        .map(|(pos, ch)| (pos, ch as usize - b'0' as usize))
        .collect::<Grid>();

    grid.positions()
        .map(|pos| grid.scenic_score(pos))
        .max()
        .unwrap_or(0)
}

struct Grid {
    trees: HashMap<(usize, usize), usize>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new() -> Self {
        Self {
            trees: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    fn set(&mut self, (x, y): (usize, usize), tree: usize) {
        self.width = self.width.max(x + 1);
        self.height = self.height.max(y + 1);
        self.trees.insert((x, y), tree);
    }

    fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        let height = self.height;

        (0..self.width).flat_map(move |x| (0..height).map(move |y| (x, y)))
    }

    fn scenic_score(&self, pos: (usize, usize)) -> usize {
        self.viewing_distance(pos, Direction::Up)
            * self.viewing_distance(pos, Direction::Down)
            * self.viewing_distance(pos, Direction::Left)
            * self.viewing_distance(pos, Direction::Right)
    }

    fn viewing_distance(&self, pos: (usize, usize), direction: Direction) -> usize {
        let Some(tree) = self.trees.get(&pos).copied() else {
            return 0
        };

        self.swath(pos, direction)
            .take_until(|other_tree| tree <= *other_tree)
            .count()
    }

    fn swath(
        &self,
        mut pos: (usize, usize),
        direction: Direction,
    ) -> impl Iterator<Item = usize> + '_ {
        iter::from_fn(move || {
            pos = direction.apply(pos)?;

            if self.is_out_of_bounds(pos) {
                return None;
            }

            self.trees.get(&pos).copied()
        })
    }

    fn is_out_of_bounds(&self, (x, y): (usize, usize)) -> bool {
        x >= self.width || y >= self.height
    }
}

impl FromIterator<((usize, usize), usize)> for Grid {
    fn from_iter<T: IntoIterator<Item = ((usize, usize), usize)>>(iter: T) -> Self {
        let mut grid = Grid::new();

        for (pos, tree) in iter {
            grid.set(pos, tree);
        }

        grid
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some(match self {
            Direction::Up => (x, y.checked_sub(1)?),
            Direction::Down => (x, y + 1),
            Direction::Left => (x.checked_sub(1)?, y),
            Direction::Right => (x + 1, y),
        })
    }
}
