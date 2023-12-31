use std::{collections::HashSet, fs, str::FromStr};

use regex::Regex;

#[derive(Debug)]
struct AssingmentReadingError;
#[derive(Debug)]
struct LineReadingError;

#[derive(Debug)]
struct Assingment {
    left: usize,
    right: usize,
}

impl FromStr for Assingment {
    type Err = AssingmentReadingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = Regex::new(r"(\d*)-(\d*)").unwrap();
        let Some((_, [left, right])) = m.captures(s).map(|caps| caps.extract()) else {
            panic!("Hay una asignacion mal")
        };
        Ok(Assingment {
            left: left.parse::<usize>().unwrap(),
            right: right.parse::<usize>().unwrap(),
        })
    }
}
impl Assingment {
    fn fully_containing(&self, other: &Self) -> bool {
        let Assingment {
            left: left_self,
            right: right_self,
        } = self;
        let Assingment {
            left: left_other,
            right: right_other,
        } = other;
        let clause_a: HashSet<usize> = (*left_other..=*right_other).collect();
        let clause_b: HashSet<usize> = (*left_self..=*right_self).collect();
        clause_a.intersection(&clause_b).count() != 0
    }
}

#[derive(Debug)]
struct Line(Assingment, Assingment);

impl FromStr for Line {
    type Err = LineReadingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = Regex::new(r"(.*),(.*)").unwrap();
        let Some((_, [left, right])) = m.captures(s).map(|caps| caps.extract()) else {
            panic!("Hay una linea mal")
        };
        Ok(Line(
            left.parse::<Assingment>().unwrap(),
            right.parse::<Assingment>().unwrap(),
        ))
    }
}

fn load_data(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

pub fn run() {
    let data = load_data("src/input");
    let overlapping_intervals: usize = data
        .lines()
        .map(|l| l.parse::<Line>().unwrap())
        .filter(|Line(lha, rha)| lha.fully_containing(&rha))
        .count();
    println!("{overlapping_intervals}");
}
