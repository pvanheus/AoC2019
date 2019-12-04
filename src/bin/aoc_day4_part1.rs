use std::env;

fn digit(num: i32, digit: u32) -> i32 {
    num / (10_i32.pow(digit-1)) - (num/10_i32.pow(digit) * 10)
}

fn never_decreasing(num: i32) -> bool {
    for i in 1..6 {
        for j in (i+1)..=6 {
            if digit(num, j) > digit(num, i) {
                return false
            }
        }
    }
    true
}

fn duplicate_digit(num: i32) -> bool {
    for i in 1..6 {
        for j in (i+1)..=6 {
            if digit(num, i) == digit(num, j) {
                return true
            }
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let parts: Vec<&str> = args[1].split('-').collect();
    let from = parts[0].parse::<i32>().unwrap();
    let to = parts[1].parse::<i32>().unwrap();

    let mut possible_password_count = 0;
    for num in from..=to {
        if never_decreasing(num) && duplicate_digit(num) {
//            eprintln!("{}", num);
            possible_password_count += 1;
        }
    }
    println!("{}", possible_password_count);
}