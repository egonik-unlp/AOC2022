
use regex::Regex;
use std::{fs, str::FromStr, vec};

#[derive(Debug)]
struct BuildPileErr;
#[derive(Debug)]
struct Crate {
    letter: char,
}
#[derive(Debug)]
struct Pile(Vec<Option<Crate>>);

impl FromStr for Pile {
    type Err = BuildPileErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut retvec: Vec<Option<Crate>> = vec![];
        let mut chunk_iterator = s.chars().array_chunks::<4>();
        while let Some(chunk) = chunk_iterator.next() {
            let retnode = match chunk.iter().collect::<String>().as_str() {
                "    " | "   " => None,
                c => Some(Crate {
                    letter: c.chars().nth(1).unwrap(),
                }),
            };
            retvec.push(retnode)
        }
        let rest = chunk_iterator.into_remainder().unwrap().collect::<String>();

        let rest = match rest.as_str() {
            "   " => None,
            c => Some(Crate {
                letter: c.chars().nth(1).unwrap(),
            }),
        };
        retvec.push(rest);
        Ok(Pile(retvec))
    }
}

fn read_data(path: &str) -> (String, String) {
    let st = fs::read_to_string(path).unwrap();
    let (stacks, moves) = st.split_once("\n\n").unwrap();
    (stacks.to_owned(), moves.to_owned())
}

pub fn run(path: &str) {
    let (stacks, moves) = read_data(path);
    let mut stack_iterator = stacks.lines().rev();
    let _ = stack_iterator.next().unwrap(); // La primera linea del iterador no me interesa (en realidad es la ultima )
    let re = Regex::new(r"\s\s\s").unwrap();
    for line in stack_iterator {
        let pile: Pile = line.parse().unwrap();
        println!("Line => {}", line);
        println!("Pile => {:?}", pile);
    }
}
