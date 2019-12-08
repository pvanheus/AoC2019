extern crate permutohedron;

use std::fs;
use std::env;
use std::cmp;

use permutohedron::LexicalPermutation;

struct IntCode {
    memory: Vec<i32>,
    ip: usize,
    output: i32,
    input: Vec<i32>,
    halted: bool
}

impl IntCode {
    fn new() -> IntCode {
        IntCode {
            memory: Vec::<i32>::new(),
            ip: 0,
            output: 0,
            input: vec![],
            halted: false
        }
    }
}

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

fn run_intcode(machine: &mut IntCode) -> i32 {
    let memory = &mut machine.memory;
    let mut start = machine.ip;
    let memory_len = memory.len();
//    let mut start = 0;
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
            memory[dest_addr] = machine.input.remove(0);
            start += 2;
        } else if op == 4 {
            // 4: output
            let source_addr = memory[start + 1] as usize;
            // do we need parameter decode here? output could be position or immediate
            machine.output = memory[source_addr];
            start += 2;
            machine.ip = start;
            return machine.output
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
    machine.halted = true;
    machine.ip = start;
    return machine.output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let line = fs::read_to_string(filename).unwrap();
    let memory: Vec<i32> = line[0..(line.len()-1)].split(',').map(|el| { el.parse().unwrap() }).collect();

    let mut phases = [5,6,7,8,9];
    let mut max_output = 0;
    loop { // loop over every permutation of phases
        let mut serial:i32; // signal carried from one machine to the next
        let mut machines: Vec<IntCode> = Vec::with_capacity(5);
        for machine_num in 0..5 {
            let mut machine = IntCode::new();
            machine.memory = memory.clone();
            machine.ip = 0;
            machine.halted = false;
            machine.input = vec![phases[machine_num], 0];
            machines.push(machine);
        }
        let mut machine_num = 0;
        loop { // loop over all the machines, carrying signal from one to the next, until the last machine halts
            let machine = &mut machines[machine_num];
            serial = run_intcode(machine);
            if machine.halted && machine_num == 4 { // the last machine has halted
                break
            } else {
                machine_num = if machine_num == 4 { 0 } else { machine_num + 1};
                assert!(machines[machine_num].input.len() == 2 || machines[machine_num].input.len() == 0);
                if machines[machine_num].input.len() == 0 { // we're past the first round of execution
                    machines[machine_num].input = vec![serial];
                } else { // we are on the first round of execution where each machine gets two inputs
                    machines[machine_num].input[1] = serial;
                }
            }
        }
        max_output = cmp::max(max_output, serial);
        if !phases.next_permutation() {
            break;
        }
    }
    println!("{}", max_output);
}