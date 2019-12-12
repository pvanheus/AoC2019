extern crate num;

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt;
use std::str::FromStr;

use num::integer;

struct Moon {
    id: usize,
    position: Vec<i32>,
    velocity: Vec<i32>
}

struct MoonSet {
    moons: Vec<Moon>
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Moon({}: {}, {}, {}: {}, {}, {})",
               self.id,
               self.position[0], self.position[1], self.position[2],
               self.velocity[0], self.velocity[1], self.velocity[2]
        )
    }
}

impl Moon {
    fn potential_energy(&self) -> i32 {
        self.position.iter().map(|x| x.abs()).sum()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.iter().map(|x| x.abs()).sum()
    }
}

fn read_moon(line: &str, id: usize) -> Moon {
    let parts: Vec<&str> = line.split(",").collect();
    let x_part = parts[0];
    let x = i32::from_str(&x_part[3..]).unwrap();
    let y_part = parts[1];
    let y = i32::from_str(&y_part[3..]).unwrap();
    let z_part = parts[2];
    let z = i32::from_str(&z_part[3..z_part.len()-1]).unwrap();
    Moon { id, position: vec![x, y, z], velocity: vec![0; 3]}
}

fn read_moons(filename: &str) -> MoonSet {
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut moons = MoonSet { moons: Vec::<Moon>::new() };
    for (moon_id, line) in reader.lines().enumerate() {
        let moon = read_moon(&line.unwrap(), moon_id);
        moons.moons.push(moon);
    }
    moons
}

fn update_velocities(moonset: &mut MoonSet) -> Vec<(usize, usize, i32)> {
    let mut changes = Vec::<(usize, usize, i32)>::new();
    for i in 0..moonset.moons.len() {
        for j in i+1..moonset.moons.len() {
            let position1 = &moonset.moons[i].position;
            let position2 = &moonset.moons[j].position;
            for dim in 0..3 {
                if position1[dim] < position2[dim] {
                    changes.push((i, dim, 1));
                    changes.push((j, dim, -1));
                } else if position1[dim] > position2[dim] {
                    changes.push((i, dim, -1));
                    changes.push((j, dim, 1));
                }
            }
        }
    }
    for change in changes.iter() {
        moonset.moons[change.0].velocity[change.1] += change.2;
    }
    changes
}

fn update_positions(moonset: &mut MoonSet) {
    for i in 0..moonset.moons.len() {
        for dim in 0..3 {
            moonset.moons[i].position[dim] += moonset.moons[i].velocity[dim];
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut moons = read_moons(filename);
    let mut periods: Vec<i64> = vec![0; 3];
    let mut found_period = vec![false; 3];
    let mut count = 0;
    loop {
        for i in 0..3 {
            if count > 0 && !found_period[i] && moons.moons.iter().fold(true, |acc, x| acc && x.velocity[i] == 0) {
                periods[i] = count;
                found_period[i] = true;
            }
        }
        if found_period.iter().fold(true, |acc, x| acc && *x) == true {
            break;
        }
        update_velocities(&mut moons);
        update_positions(&mut moons);
        count += 1;
    }

    println!("{}", periods.iter().fold(1, |acc, x| integer::lcm(acc, *x)) * 2);
}