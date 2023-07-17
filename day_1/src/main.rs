use std::fs;

fn readfile(path : &str) -> Result<Vec<Option<i32>>, Box< dyn std::error::Error>> {
    let calories = fs::read_to_string(path)?.split('\n').into_iter().map(|n| n.parse::<i32>().ok()).collect();
    Ok(calories)
}

fn main() {
    let mut results : Vec<i32> = vec![];
    let calories = readfile("./src/input").unwrap();
    let mut accum = 0i32;
    for cal_value in calories.iter() {
        match cal_value {
            Some(value) => accum += value,
            None => {
                results.push(accum);
                accum = 0;
            }
        }
    }
    if accum != 0 {
        results.push(accum)
    }
    results.sort();
    let topthree : i32 = results.iter().rev().take(3).sum(); 
    println!("{:?}",topthree)
}
