use aoc::aoc;

#[aoc(2022, 2, 2)]
fn main(input: &str) -> u32 {
    input.lines()
        .map(|round| Round::parse(round).player_score())
        .sum()
}

struct Round {
    opponent: Shape,
    desired_outcome: Outcome,
}

impl Round {
    fn parse(s: &str) -> Self {
        Self {
            opponent: Shape::parse(&s[..1]),
            desired_outcome: Outcome::parse(&s[2..]),
        }
    }

    fn player_score(&self) -> u32 {
        let player_shape = self.player_shape();
        let shape_score = player_shape.score();
        let outcome_score = player_shape.against(&self.opponent).score();

        shape_score + outcome_score
    }

    fn player_shape(&self) -> Shape {
        match (&self.opponent, &self.desired_outcome) {
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Paper, Outcome::Lost) => Shape::Rock,
            (Shape::Scissors, Outcome::Won) => Shape::Rock,
            (Shape::Rock, Outcome::Won) => Shape::Paper,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Scissors, Outcome::Lost) => Shape::Paper,
            (Shape::Rock, Outcome::Lost) => Shape::Scissors,
            (Shape::Paper, Outcome::Won) => Shape::Scissors,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
        }
    }
}

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn parse(s: &str) -> Self {
        match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn against(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Rock) => Outcome::Draw,
            (Self::Rock, Self::Paper) => Outcome::Lost,
            (Self::Rock, Self::Scissors) => Outcome::Won,
            (Self::Paper, Self::Rock) => Outcome::Won,
            (Self::Paper, Self::Paper) => Outcome::Draw,
            (Self::Paper, Self::Scissors) => Outcome::Lost,
            (Self::Scissors, Self::Rock) => Outcome::Lost,
            (Self::Scissors, Self::Paper) => Outcome::Won,
            (Self::Scissors, Self::Scissors) => Outcome::Draw,
        }
    }
}

enum Outcome {
    Lost,
    Draw,
    Won,
}

impl Outcome {
    fn parse(s: &str) -> Self {
        match s {
            "X" => Self::Lost,
            "Y" => Self::Draw,
            "Z" => Self::Won,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Lost => 0,
            Self::Draw => 3,
            Self::Won => 6,
        }
    }
}
