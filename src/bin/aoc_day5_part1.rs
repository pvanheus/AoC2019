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
            assert!(modes[2] == 0);
            let a = if modes[0] == 0 { memory[memory[start+1] as usize] } else { memory[start+1] };
            let b = if modes[1] == 0 { memory[memory[start+2] as usize] } else { memory[start+2] };
            let dest_addr = memory[start+3] as usize;
            if op == 1 {
                memory[dest_addr] = a + b;
            } else if op == 2 {
                memory[dest_addr] = a * b;
            }
            start += 4;
        } else if op == 3 {
            let dest_addr = memory[start + 1] as usize;
            memory[dest_addr] = inputs[input_counter];
            input_counter += 1;
            start += 2;
        } else if op == 4 {
            let source_addr = memory[start+1] as usize;
            // do we need parameter decode here? output could be position or immediate
            println!("OUTPUT: {} {}", output_count, memory[source_addr]);
            output_count += 1;
            start += 2;
        } else {
            println!("{} {}", op, memory[start]);
            panic!("unknown operation!")
        }
        decoded = decode_op(memory[start]);
        op = decoded.0;
        modes = decoded.1;

    }
    eprintln!("{:?}", memory);
    memory[0]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let line = fs::read_to_string(filename).unwrap();
    let memory: Vec<i32> = line[0..(line.len()-1)].split(',').map(|el| { el.parse().unwrap() }).collect();
    run_intcode(memory, vec![1]);
}