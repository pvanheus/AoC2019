/*
0123456012345
<x=-14, y=-4, z=-11>
<x=-9, y=6, z=-7>
<x=4, y=1, z=4>
<x=2, y=-14, z=-9>
*/
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt;
use std::str::FromStr;

struct Moon {
    id: usize,
    position: Vec<i32>,
    velocity: Vec<i32>
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

fn read_moons(filename: &str) -> Vec<Moon> {
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut moons = Vec::<Moon>::new();
    for (moon_id, line) in reader.lines().enumerate() {
        let moon = read_moon(&line.unwrap(), moon_id);
        moons.push(moon);
    }
    moons
}

fn update_velocities(moons: &mut Vec<Moon>) -> Vec<(usize, usize, i32)> {
    let mut changes = Vec::<(usize, usize, i32)>::new();
    for i in 0..moons.len() {
        for j in i+1..moons.len() {
            let position1 = &moons[i].position;
            let position2 = &moons[j].position;
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
        moons[change.0].velocity[change.1] += change.2;
    }
    changes
}

fn update_positions(moons: &mut Vec<Moon>) {
    for moon in moons {
        for dim in 0..3 {
            moon.position[dim] += moon.velocity[dim];
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut moons = read_moons(filename);
    for _ in 0..1000 {
        let changes = update_velocities(&mut moons);
//        for moon in moons.iter() {
//            eprintln!("{}", moon);
//        }
        update_positions(&mut moons);
        for moon in moons.iter() {
            eprintln!("{}", moon);
        }
        eprintln!();
    }

    let total_energy: i32 = moons.iter().map(|moon| moon.kinetic_energy() * moon.potential_energy()).sum();
    println!("{}", total_energy);
}