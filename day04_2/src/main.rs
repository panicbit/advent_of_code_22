use aoc::aoc;

#[aoc(2022, 4, 2)]
fn main(input: &str) -> usize {
    input
        .lines()
        .map(|pair| {
            let (range1, range2) = pair.split_once(',').unwrap();
            let range1 = Range::parse(range1);
            let range2 = Range::parse(range2);

            (range1, range2)
        })
        .filter(|(range1, range2)| range1.overlaps_with(range2))
        .count()
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn parse(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();

        Self {
            start: start.parse::<u32>().unwrap(),
            end: end.parse::<u32>().unwrap(),
        }
    }

    fn contains(&self, value: u32) -> bool {
        self.start <= value && value <= self.end
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        self.contains(other.start)
            || self.contains(other.end)
            || other.contains(self.start)
            || other.contains(self.end)
    }
}
