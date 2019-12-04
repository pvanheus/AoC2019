use std::fs;
use std::env;

fn run_intcode(mut memory: Vec<i32>, noun: i32, verb: i32) -> i32 {
    memory[1] = noun;
    memory[2] = verb;

    let memory_len = memory.len();
    let mut start = 0;
    let op = memory[start];
    while op != 99 && start < memory_len {
        let op = memory[start];
        if op == 99 {
            break;
        }
        let a_addr = memory[start+1] as usize;;
        let b_addr = memory[start+2] as usize;
        let dest_addr = memory[start+3] as usize;
        if op == 1 {
            memory[dest_addr] = memory[a_addr] + memory[b_addr];
        } else if op == 2 {
            memory[dest_addr] = memory[a_addr] * memory[b_addr];
        } else if op == 99 {
            break;
        } else {
            panic!("unknown operation!")
        }
        start += 4;
    }
    memory[0]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let line = fs::read_to_string(filename).unwrap();
    let memory: Vec<i32> = line[0..(line.len()-1)].split(',').map(|el| { el.parse().unwrap() }).collect();
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = run_intcode(memory.clone(), noun, verb);
            if result == 19690720 {
                println!("{} {} {}", 100 * noun + verb, noun, verb);
                break;
            }
        }
    }
}