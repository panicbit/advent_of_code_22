use std::collections::HashMap;
use std::iter;

use aoc::aoc;

#[aoc(2022, 8, 1)]
fn main(input: &str) -> usize {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, ch)| ((x, y), ch)))
        .map(|(pos, ch)| (pos, ch as usize - b'0' as usize))
        .collect::<Grid>();

    grid.positions()
        .filter(|pos| grid.is_visible(*pos))
        .count()
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

    fn is_visible(&self, pos: (usize, usize)) -> bool {
        if self.is_edge(pos) {
            return true;
        }

        let Some(tree) = self.trees.get(&pos) else {
            return false;
        };

        self.swaths(pos)
            .any(|mut swath| swath.all(|other_tree| tree > &other_tree))
    }

    fn is_edge(&self, (x, y): (usize, usize)) -> bool {
        x == 0 || y == 0 || x + 1 == self.width || y + 1 == self.height
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

    fn swaths(
        &self,
        pos: (usize, usize),
    ) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> + '_ {
        [
            self.swath(pos, Direction::Up),
            self.swath(pos, Direction::Down),
            self.swath(pos, Direction::Left),
            self.swath(pos, Direction::Right),
        ]
        .into_iter()
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
