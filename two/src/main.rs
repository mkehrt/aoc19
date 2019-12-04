use std::{env, fs};

fn execute(mem: &mut Vec<i32>) {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let contents = fs::read_to_string(input).unwrap();

    let mut mem = contents
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    execute(&mut mem);

    println!("{}", mem[0]);
}
