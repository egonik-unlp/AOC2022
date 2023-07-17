use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn load_data() -> String {
    fs::read_to_string("./src/input").unwrap()
}
fn make_score_database() -> HashMap<char, usize> {
    String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect())
        .unwrap() // No va a fallar, no pasa nada dejarlo asi
        .chars()
        .into_iter()
        .enumerate()
        .map(|(x, c)| (c, x + 1))
        .collect::<HashMap<_, _>>()
}

pub fn run() {
    let scores = make_score_database();
    let data = load_data();
    let final_score = data
        .lines()
        .into_iter()
        .map(|line| {
            let halflinelen = line.len() / 2;
            let (first_half, second_half) = (&line[..halflinelen], &line[halflinelen..]);
            let hash1: HashSet<char> = first_half.to_owned().chars().collect();
            let hash2: HashSet<char> = second_half.to_owned().chars().collect();
            hash1.intersection(&hash2).nth(0).unwrap().clone()
        })
        .map(|character| scores.get(&character).unwrap())
        .sum::<usize>();
    println!("{final_score}");
}
