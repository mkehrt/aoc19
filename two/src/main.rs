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

    let initial_mem: Vec<i32> = contents
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    'outer: for noun in 1..100 {
        for verb in 1..100 {
            let mut mem = initial_mem.clone();

            mem[1] = noun;
            mem[2] = verb;

            execute(&mut mem);

            let output = mem[0];

            if output == 19690720 {
                println!("{:}", noun * 100 + verb);
                break 'outer;
            }
        }
    }
}
