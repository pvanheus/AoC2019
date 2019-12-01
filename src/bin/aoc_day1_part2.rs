extern crate math;

use math::round;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn compute_fuel(mass: f64) -> f64 {
    return round::floor(mass / 3.0, 0) - 2.0;
}

fn main() {
    let filename = "data/aoc1.txt";
    let input_file = File::open(filename).unwrap();
    let reader = BufReader::new(input_file);
    let mut total_fuel = 0.0;
    for maybe_line in reader.lines() {
        match maybe_line {
            Ok(line) => {
                let mass = line.parse::<f64>().unwrap();
                let fuel = compute_fuel(mass);
                total_fuel += fuel;
                let mut mass_of_fuel = compute_fuel(fuel);
                while mass_of_fuel >= 0.0 {
                    total_fuel += mass_of_fuel;
                    mass_of_fuel = compute_fuel(mass_of_fuel);
                }
            },
            _ => eprintln!("Failed to read a line from the reader")
        }
    }
    println!("{}", total_fuel);
}
