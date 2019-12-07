use std::fs;
use std::env;

fn digit(num: i32, digit: u32) -> i32 {
    num / (10_i32.pow(digit-1)) - (num/10_i32.pow(digit) * 10)
}

fn decode_op(instruction: i32) -> (i32, Vec<i32>) {
    let width = 1 + (instruction as f64).log(10_f64) as u32;
    let op = digit(instruction, 2) * 10 + digit(instruction, 1);
    let mut modes: Vec<i32> = vec![0, 0, 0];
    let mut mode_index = 0;
    for i in 3..=width {
        modes[mode_index] = digit(instruction, i);
        mode_index += 1;
    }
    (op, modes)
}

fn fetch(mode: i32, addr: usize, memory: &Vec<i32>) -> i32 {
    if mode == 0 { memory[memory[addr] as usize] } else { memory[addr] }
}

fn run_intcode(mut memory: Vec<i32>, inputs: Vec<i32>) -> i32 {
    let mut input_counter = 0;
    let mut output_count = 0;
    let memory_len = memory.len();
    let mut start = 0;
    // should probably use a struct here
    let mut decoded = decode_op(memory[start]);
    let mut op = decoded.0;
    let mut modes = decoded.1;
    while op != 99 && start < memory_len {
        if op == 99 {
            break;
        } else if op == 1 || op == 2 {
            // 1: add, 2: multiply
            let a = fetch(modes[0], start+1, &memory);
            let b = fetch(modes[1], start+2, &memory);
            let dest_addr = memory[start+3] as usize;
            if op == 1 {
                memory[dest_addr] = a + b;
            } else if op == 2 {
                memory[dest_addr] = a * b;
            }
            start += 4;
        } else if op == 3 {
            // 3: input
            let dest_addr = memory[start + 1] as usize;
            memory[dest_addr] = inputs[input_counter];
            input_counter += 1;
            start += 2;
        } else if op == 4 {
            // 4: output
            let source_addr = memory[start + 1] as usize;
            // do we need parameter decode here? output could be position or immediate
            println!("OUTPUT: {} {}", output_count, memory[source_addr]);
            output_count += 1;
            start += 2;
        } else if op == 5 || op == 6 {
            // 5: jmp if not zero, 6: jmp if not zero
            let a = fetch(modes[0], start+1, &memory);
            let dest_addr = fetch(modes[1], start+2, &memory);
            if (op == 5 && a != 0) || (op == 6 && a == 0) {
                start = dest_addr as usize;
            } else {
                start += 3;
            }
        } else if op == 7 || op == 8 {
            // 7: set dest to 1 if < else set to 0, 8: set dest to 1 if == else set to 0
            // need to not use duplicated code here
            let a = fetch(modes[0], start+1, &memory);
            let b = fetch(modes[1], start+2, &memory);
            let dest_addr = memory[start + 3] as usize;
            if (op == 7 && a < b) || (op == 8 && a == b) {
                memory[dest_addr] = 1;
            } else {
                memory[dest_addr] = 0;
            }
            start += 4;
        } else {
            eprintln!("{}", op);
            eprintln!("{} {:?}", start, memory);
            panic!("unknown operation!")
        }

        decoded = decode_op(memory[start]);
        op = decoded.0;
        modes = decoded.1;
    }
    memory[0]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let line = fs::read_to_string(filename).unwrap();
    let memory: Vec<i32> = line[0..(line.len()-1)].split(',').map(|el| { el.parse().unwrap() }).collect();
    run_intcode(memory, vec![5]);
}