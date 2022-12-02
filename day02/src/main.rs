use aoc::aoc;

#[aoc(2022, 2, 1)]
fn main(input: &str) -> u32 {
    input.lines()
        .map(|round| Round::parse(round).player_score())
        .sum()
}

struct Round {
    opponent: Shape,
    player: Shape,
}

impl Round {
    fn parse(s: &str) -> Self {
        Self {
            opponent: Shape::parse(&s[..1]),
            player: Shape::parse(&s[2..]),
        }
    }

    fn player_score(&self) -> u32 {
        let shape_score = self.player.score();
        let outcome_score = self.player.against(&self.opponent).score();

        shape_score + outcome_score
    }
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn parse(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
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
    fn score(&self) -> u32 {
        match self {
            Self::Lost => 0,
            Self::Draw => 3,
            Self::Won => 6,
        }
    }
}
