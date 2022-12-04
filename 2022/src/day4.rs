use std::error::Error;
use std::fs;
use std::result::Result;
use std::vec::Vec;

#[derive(Debug)]
struct Range {
    start: u8,
    end: u8,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let mut split = s.split('-');
        let start: u8 = split.next().unwrap().parse().unwrap();
        let end: u8 = split.next_back().unwrap().parse().unwrap();

        Self { start, end }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file_content = fs::read_to_string("input/day4.txt")?;

    let pairs: Vec<_> = input_file_content
        .lines()
        .map(|line| {
            let mut pairs = line.split(',');
            let first: Range = pairs.next().unwrap().into();
            let second: Range = pairs.next_back().unwrap().into();
            (first, second)
        })
        .collect();

    let part1_count = pairs
        .iter()
        .filter(|(first, second)| first.contains(&second) || second.contains(&first))
        .count();

    println!("{:?}", part1_count);

    let part2_count = pairs
        .iter()
        .filter(|(first, second)| first.overlaps(second))
        .count();

    println!("{:?}", part2_count);

    Ok(())
}
