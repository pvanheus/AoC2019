pub struct IntCode {
    pub memory: Vec<i64>,
    pub ip: usize,
    pub output: i64,
    pub input: Vec<i64>,
    pub halted: bool,
    pub relative_base: i64
}

impl IntCode {
    pub fn new() -> IntCode {
        IntCode {
            memory: Vec::<i64>::new(),
            ip: 0,
            output: 0,
            input: vec![],
            halted: false,
            relative_base: 0
        }
    }
}

fn digit(num: i64, digit: u32) -> i64 {
    num / (10_i64.pow(digit-1)) - (num/10_i64.pow(digit) * 10)
}

fn decode_op(instruction: i64) -> (i64, Vec<i64>) {
    let width = 1 + (instruction as f64).log(10_f64) as u32;
    let op = digit(instruction, 2) * 10 + digit(instruction, 1);
    let mut modes: Vec<i64> = vec![0, 0, 0];
    let mut mode_index = 0;
    for i in 3..=width {
        modes[mode_index] = digit(instruction, i);
        mode_index += 1;
    }
    (op, modes)
}

fn fetch(mode: i64, addr: usize, memory: &Vec<i64>, relative_base: i64) -> i64 {
    if mode == 0 { memory[memory[addr] as usize] } else if mode == 1 { memory[addr] } else { memory[(memory[addr] + relative_base) as usize] }
}

fn put(value: i64, mode: i64, addr: usize, memory: &mut Vec<i64>, relative_base: i64) {
    assert!(mode != 1); // can't put to an immediate mode address
    let dest_addr = if mode == 0 { memory[addr] as usize} else { (memory[addr] + relative_base) as usize };
    memory[dest_addr] = value;
}

pub fn run_intcode(machine: &mut IntCode) -> i64 {
    let memory = &mut machine.memory;
    let mut start = machine.ip;
    let memory_len = memory.len();
//    let mut start = 0;
    // should probably use a struct here
    let mut decoded = decode_op(memory[start]);
    let mut op = decoded.0;
    let mut modes = decoded.1;
    while op != 99 && start < memory_len {
//        println!("{}", op);
        if op == 99 {
            break;
        } else if op == 1 || op == 2 {
            // 1: add, 2: multiply
            let a = fetch(modes[0], start+1, &memory, machine.relative_base);
            let b = fetch(modes[1], start+2, &memory, machine.relative_base);
            if op == 1 {
                put(a + b, modes[2], start+3, memory, machine.relative_base);
            } else if op == 2 {
                put(a * b, modes[2], start+3, memory, machine.relative_base);
            }
            start += 4;
        } else if op == 3 {
            // 3: input
            put(machine.input.remove(0), modes[0], start + 1, memory, machine.relative_base);
            start += 2;
        } else if op == 4 {
            // 4: output
            let output = fetch(modes[0], start+1, &memory, machine.relative_base);
//            println!("SA: {} {} {} {:?}", output, memory[start+1], machine.relative_base, modes);
            machine.output = output;
//            eprint!("O{}O ", machine.output);
            start += 2;
            machine.ip = start;
            return machine.output
        } else if op == 5 || op == 6 {
            // 5: jmp if not zero, 6: jmp if not zero
            let a = fetch(modes[0], start+1, &memory, machine.relative_base);
            let dest_addr = fetch(modes[1], start+2, &memory, machine.relative_base);
            if (op == 5 && a != 0) || (op == 6 && a == 0) {
                start = dest_addr as usize;
            } else {
                start += 3;
            }
        } else if op == 7 || op == 8 {
            // 7: set dest to 1 if < else set to 0, 8: set dest to 1 if == else set to 0
            // need to not use duplicated code here
            let a = fetch(modes[0], start + 1, &memory, machine.relative_base);
            let b = fetch(modes[1], start + 2, &memory, machine.relative_base);
            if (op == 7 && a < b) || (op == 8 && a == b) {
                put(1, modes[2], start + 3, memory, machine.relative_base);
            } else {
                put(0, modes[2], start + 3, memory, machine.relative_base);
            }
            start += 4;
        } else if op == 9 {
            machine.relative_base += fetch(modes[0], start+1, &memory, machine.relative_base);
            start += 2;
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
//    println!();
    return machine.output
}
