use std::collections::HashMap;

use aoc::aoc;
use pathfinding::directed::astar::astar;

#[aoc(2022, 12, 2)]
fn main(input: &str) -> usize {
    let grid = Grid::parse(input);

    grid.info.iter()
        .filter(|(_, info)| **info == Info::Height('a'))
        .flat_map(|(pos, _)| grid.shortest_path_to_end(pos))
        .min()
        .unwrap()
}

struct Grid {
    info: HashMap<(usize, usize), Info>,
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Info {
    Start,
    Height(char),
    End,
}

impl Info {
    fn parse(ch: char) -> Self {
        if ch == 'S' {
            return Self::Start;
        }

        if ch == 'E' {
            return Self::End;
        }

        Self::Height(ch)
    }

    fn elevation(&self) -> u8 {
        match self {
            Info::Start => b'a',
            Info::Height(ch) => *ch as u8,
            Info::End => b'z',
        }
    }
}

impl Grid {
    fn new() -> Self {
        Self {
            info: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    fn parse(s: &str) -> Self {
        s.lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, ch)| ((x, y), ch)))
            .map(|(pos, ch)| (pos, Info::parse(ch)))
            .collect::<Grid>()
    }

    fn set(&mut self, (x, y): (usize, usize), info: Info) {
        self.width = self.width.max(x + 1);
        self.height = self.height.max(y + 1);
        self.info.insert((x, y), info);
    }

    fn climbable_neighbours(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let info = &self.info[&pos];

        [
            Direction::Up.apply(pos),
            Direction::Down.apply(pos),
            Direction::Left.apply(pos),
            Direction::Right.apply(pos),
        ]
        .into_iter()
        .flatten()
        .filter(|pos| {
            let Some(other_info) = self.info.get(pos).copied() else {
                return false;
            };

            if info.elevation() < other_info.elevation() && (other_info.elevation() - info.elevation()) > 1 {
                return false;
            }

            true
        })
    }

    fn shortest_path_to_end(&self, pos: &(usize, usize)) -> Option<usize> {
        let (_, steps) = astar(
            pos,
            |pos| self.climbable_neighbours(*pos).map(|pos| (pos, 1)),
            |_pos| 1,
            |pos| self.info.get(pos) == Some(&Info::End),
        )?;

        Some(steps)
    }
}

impl FromIterator<((usize, usize), Info)> for Grid {
    fn from_iter<T: IntoIterator<Item = ((usize, usize), Info)>>(iter: T) -> Self {
        let mut grid = Grid::new();

        for (pos, info) in iter {
            grid.set(pos, info);
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
