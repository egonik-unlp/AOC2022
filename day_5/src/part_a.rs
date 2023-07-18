use std::fs;

fn read_data(path : &str) -> (String, String) {
	let st = fs::read_to_string(path).unwrap();
	let (stacks, moves) = st.split_once("\n\n").unwrap();
	(stacks.to_owned(), moves.to_owned())
}


pub fn run(path : &str) {
	let (stacks, moves) = read_data(path);
	let mut it_ch =  stacks.lines().rev().array_chunks::<9>();
	let numm_code = it_ch.next().unwrap();
	println!("Fuera del for =>  {:?}", numm_code);
	for block in it_ch {
		println!("Dentro del for => {:?}", block)
	}
}