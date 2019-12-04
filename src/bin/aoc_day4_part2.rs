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

fn only_duplicate_digit(num: i32) -> bool {
    for i in 1..=5 {
        if digit(num, i) == digit(num, i+1) {
            if !triplicate_digit(num, digit(num, i)) {
                return true
            }
        }
    }
    false
}

fn triplicate_digit(num: i32, target: i32) -> bool {
    for i in 1..=4 {
        if digit(num, i) == target && digit(num, i) == digit(num, i+1) && digit(num, i) == digit(num, i+2) {
            return true
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let parts: Vec<&str> = args[1].split('-').collect();
    let from = parts[0].parse::<i32>().unwrap();
    let to = parts[1].parse::<i32>().unwrap();

    eprintln!("test some logic for duplicated digits");
    eprintln!("while some patterns like 112111 would confuse the code they are excluded by the not-decreasing rule");
    eprintln!("112233 {}", only_duplicate_digit(112233));
    eprintln!("123444 {}", only_duplicate_digit(123444));
    eprintln!("111122 {}", only_duplicate_digit(111122));
    eprintln!("124444 {}", only_duplicate_digit(124444));
    eprintln!("113444 {}", only_duplicate_digit(113444));
    eprintln!("111444 {}", only_duplicate_digit(111444));

    let mut possible_password_count = 0;
    for num in from..=to {
        if never_decreasing(num) && only_duplicate_digit(num) {
            possible_password_count += 1;
        }
    }
    println!("{}", possible_password_count);
}