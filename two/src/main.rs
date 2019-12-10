use std::{env, fs};

fn execute_binary_op(instr: &Vec<i32>, mem: &mut Vec<i32>) {
    let op = instr[0];
    let arg1 = mem[instr[1] as usize];
    let arg2 = mem[instr[2] as usize];
    let dest = &mut mem[instr[3] as usize];

    let result = match op {
        1 => arg1 + arg2,
        2 => arg1 * arg2,
        _ => panic!("Unknown binary operator {}.", op),
    };

    std::mem::replace(dest, result);
}

fn execute(mem: &mut Vec<i32>) {
    let mut pc = 0;
    loop {
        println!("{:?}", mem);
        let instr = mem[pc];
        match instr {
            99 => break,
            1 | 2 => execute_binary_op(&Vec::from(&mem[pc..pc + 4]), mem),
            _ => panic!("Unknown instruction {}.", instr),
        }
        pc += 4;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let contents = fs::read_to_string(input).unwrap();

    let mut mem: Vec<i32> = contents
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    mem[1] = 12;
    mem[2] = 2;

    execute(&mut mem);

    println!("{:?}", mem[0]);
}
