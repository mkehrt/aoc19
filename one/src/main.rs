use std::{env, fs};

fn main() {
    let args:Vec<String> = env::args().collect();
    let input = &args[1];
    let contents = fs::read_to_string(input).unwrap();

    let masses = contents.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let fuel: i32 = masses.map(|x| (x / 3) - 2).sum();
    
    println!("{}", fuel);
}
