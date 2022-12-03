use std::error::Error;
use std::fs;
use std::result::Result;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'X' | 'A' => Shape::Rock,
            'Y' | 'B' => Shape::Paper,
            'Z' | 'C' => Shape::Scissor,
            _ => panic!("unexpected move '{}", c),
        }
    }
}

impl Shape {
    fn counter(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
    }

    fn points(self, opponent_play: Shape) -> u32 {
        if self == opponent_play {
            GameResult::Draw as u32
        } else if self.counter() == opponent_play {
            GameResult::Win as u32
        } else {
            GameResult::Loss as u32
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum GameResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl From<char> for GameResult {
    fn from(c: char) -> Self {
        match c {
            'X' => GameResult::Loss,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => panic!("unexpected game result: {}", c),
        }
    }
}

impl GameResult {
    fn resolve(&self, shape: Shape) -> Shape {
        match self {
            GameResult::Win => shape.counter().counter(),
            GameResult::Draw => shape,
            GameResult::Loss => shape.counter(),
        }
    }
}

fn strategy_part1(line: &str) -> u32 {
    let mut line_chars = line.chars();
    let opponent_play: Shape = line_chars.next().unwrap().into();
    let my_play: Shape = line_chars.next_back().unwrap().into();

    my_play.points(opponent_play) + (my_play as u32)
}

fn strategy_part2(line: &str) -> u32 {
    let mut line_chars = line.chars();
    let opponent_play: Shape = line_chars.next().unwrap().into();
    let expected_game_result: GameResult = line_chars.next_back().unwrap().into();
    let my_play = expected_game_result.resolve(opponent_play);

    (expected_game_result as u32) + (my_play as u32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string("input/day2.txt")?;
    let sum_part1 = file_content.lines().map(strategy_part1).sum::<u32>();
    println!("1.part: {}", sum_part1);

    let sum_part2 = file_content.lines().map(strategy_part2).sum::<u32>();
    println!("2.part: {}", sum_part2);

    Ok(())
}
