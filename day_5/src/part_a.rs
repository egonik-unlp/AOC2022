use std::fs;
use regex::Regex;

fn read_data(path : &str) -> (String, String) {
	let st = fs::read_to_string(path).unwrap();
	let (stacks, moves) = st.split_once("\n\n").unwrap();
	(stacks.to_owned(), moves.to_owned())
}


pub fn run(path : &str) {
	let (stacks, moves) = read_data(path);
	println!("{:?}",stacks);
	let mut stack_iterator = stacks.lines().rev();
	let _ = stack_iterator.next().unwrap();
	let re = Regex::new(r"(\[\w\])*").unwrap();
	for line in stack_iterator {
		println!("l -> {:?}   le -{:?}", line, line.len());
		let Some(caps) = re.captures(line) else {
			println!("no match!");
			return;
		    };
		println!("{caps:?}");

	}
}