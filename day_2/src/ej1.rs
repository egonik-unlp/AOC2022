use split_iter::Splittable;
use std::error::Error;
use std::str::FromStr;
use std::{fs, vec};

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

struct ParseRPSError;
enum RPSResult {
    Win,
    Tie,
    Loss,
}
impl RPSResult {
    fn score(&self) -> u32 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Tie => 3,
            RPSResult::Loss => 0,
        }
    }
}

impl RPS {
    fn self_worth(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
    fn battle(&self, other: Self) -> u32 {
        let battle_result = match self {
            RPS::Rock => match other {
                RPS::Paper => RPSResult::Loss,
                RPS::Scissors => RPSResult::Win,
                RPS::Rock => RPSResult::Tie,
            },
            RPS::Paper => match other {
                RPS::Paper => RPSResult::Tie,
                RPS::Scissors => RPSResult::Loss,
                RPS::Rock => RPSResult::Win,
            },
            RPS::Scissors => match other {
                RPS::Paper => RPSResult::Win,
                RPS::Scissors => RPSResult::Tie,
                RPS::Rock => RPSResult::Loss,
            },
        };
        battle_result.score() + self.self_worth()
    }
}

impl FromStr for RPS {
    type Err = ParseRPSError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "A" => Ok(RPS::Rock),
            "Y" | "B" => Ok(RPS::Paper),
            "Z" | "C" => Ok(RPS::Scissors),
            _ => Err(ParseRPSError),
        }
    }
}
fn from_row(input: &str) -> Result<(RPS, RPS), Box<dyn Error>> {
    let mut retvals: Vec<RPS> = vec![];
    for node in 0..=2 {
        if let Ok(v) = RPS::from_str(&input.chars().nth(node).unwrap_or('J').to_string()) {
            retvals.push(v)
        }
    }
    Ok((retvals[0], retvals[1]))
}

pub fn run() {
    let match_inputs = fs::read_to_string("src/input").expect("FILE NOT FOUND");
    let (_, trows) = match_inputs.split("\n").into_iter().split(|n| n.len() > 1);
    let result = trows
        .into_iter()
        .map(|f| from_row(f).unwrap())
        .map(|(a, b)| b.battle(a))
        .sum::<u32>();
    println!("Saque {} puntos", result)
}
