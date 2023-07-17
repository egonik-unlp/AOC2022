
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
	let mut results : Vec<usize> = vec![];
	for [x,y, z] in  data.split('\n').into_iter().array_chunks() {
		let s1 : HashSet<char> = x.chars().collect();
		let s2 : HashSet<char> = y.chars().collect();
		let s3 : HashSet<char> = z.chars().collect();
		let fi : HashSet<char>  = s1.intersection(&s2).into_iter().map(|x|x.to_owned()).collect();
		let ff : HashSet<char>  = fi.intersection(&s3).into_iter().map(|x|x.to_owned()).collect();
		let common_item = ff.into_iter().nth(0).unwrap();
		results.push(*scores.get(&common_item).unwrap());
	}
	let final_result = results.into_iter().sum::<usize>();
	println!("{final_result:?}")
    }
    