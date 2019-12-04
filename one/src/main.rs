use std::{env, fs};

fn fuel_for_mass(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel > 0 {
        fuel + fuel_for_mass(fuel)
    } else {
        0
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let contents = fs::read_to_string(input).unwrap();

    let masses = contents
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());
    let fuel: i32 = masses.map(fuel_for_mass).sum();

    println!("{}", fuel);
}
