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

struct ParseIndCharError;

#[derive(Debug, Clone, Copy)]
enum RPSResult {
    Win,
    Tie,
    Loss,
}
#[derive(Debug, Clone, Copy)]
enum IndChar {
    Resultado(RPSResult),
    Pieza(RPS),
}

fn resolve(a: IndChar, b: IndChar) -> u32 {
    if let (IndChar::Pieza(a), IndChar::Resultado(b)) = (a, b) {
        let my_move = a.match_pairs(b);
        my_move.self_worth() + b.score()
    } else {
        0
    }
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

impl FromStr for IndChar {
    type Err = ParseIndCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(IndChar::Resultado(RPSResult::Loss)),
            "Y" => Ok(IndChar::Resultado(RPSResult::Tie)),
            "Z" => Ok(IndChar::Resultado(RPSResult::Win)),
            "A" => Ok(IndChar::Pieza(RPS::Rock)),
            "B" => Ok(IndChar::Pieza(RPS::Paper)),
            "C" => Ok(IndChar::Pieza(RPS::Scissors)),
            _ => Err(ParseIndCharError),
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
    fn match_pairs(&self, other: RPSResult) -> RPS {
        match self {
            RPS::Rock => match other {
                RPSResult::Win => RPS::Paper,
                RPSResult::Loss => RPS::Scissors,
                RPSResult::Tie => RPS::Rock,
            },
            RPS::Paper => match other {
                RPSResult::Tie => RPS::Paper,
                RPSResult::Win => RPS::Scissors,
                RPSResult::Loss => RPS::Rock,
            },
            RPS::Scissors => match other {
                RPSResult::Loss => RPS::Paper,
                RPSResult::Tie => RPS::Scissors,
                RPSResult::Win => RPS::Rock,
            },
        }
    }
}

fn from_row(input: &str) -> Result<(IndChar, IndChar), Box<dyn Error>> {
    let mut retvals: Vec<IndChar> = vec![];
    for node in 0..=2 {
        if let Ok(v) = IndChar::from_str(&input.chars().nth(node).unwrap_or('J').to_string()) {
            retvals.push(v)
        }
    }
    Ok((retvals[0], retvals[1]))
}

pub fn run() {
    let match_inputs = fs::read_to_string("src/input").expect("FILE NOT FOUND");
    let (_, trows) = match_inputs.split("\n").into_iter().split(|n| n.len() > 1);

    let partial_result: Vec<(IndChar, IndChar)> =
        trows.into_iter().map(|f| from_row(f).unwrap()).collect();
    let result = partial_result
        .into_iter()
        .map(|(a, b)| resolve(a, b))
        .sum::<u32>();
    println!("Saque {} puntos", result)
}
