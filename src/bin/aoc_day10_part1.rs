use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::f64::INFINITY;

#[derive(PartialEq)]
#[derive(Debug)]
struct Ray {
    direction: f64,
    side: i32
}

impl Hash for Ray {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.direction.to_string().hash(state);
        self.side.hash(state);
    }
}

impl Eq for Ray {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut asteroids = Vec::<(i32, i32)>::new();
    let mut y = 0;
    for line_maybe in reader.lines() {
        let line = line_maybe.unwrap();
        let mut x = 0;
        for char in line.chars() {
            if char == '.' || char == '#' {
                if char == '#' {
                    asteroids.push((x, y));
                }
            }
            x += 1;
        }
        y += 1;
    }

    eprintln!("total asteroids: {}", asteroids.len());
    let mut max_ray_count = 0;
    let mut best_asteroid = (0, 0);
    for asteroid in &asteroids {
        let mut rays = HashSet::<Ray>::new();
        for other_asteroid in &asteroids {
            if !(other_asteroid.0 == asteroid.0 && other_asteroid.1 == asteroid.1) {
                let tan = if (other_asteroid.1 - asteroid.1) == 0 { INFINITY } else { (other_asteroid.0 - asteroid.0) as f64 / (other_asteroid.1 - asteroid.1) as f64 };
                let direction = tan.atan().to_degrees();
//                if direction == 90.0 {
//                    eprintln!("self: {} {} other: {} {}", asteroid.0, asteroid.1, other_asteroid.0, other_asteroid.1);
//                }
                let side = if (other_asteroid.1 - asteroid.1) == 0 { (other_asteroid.0 - asteroid.0) / (other_asteroid.0 - asteroid.0).abs() } else { (other_asteroid.1 - asteroid.1) / (other_asteroid.1 - asteroid.1).abs() };
                rays.insert(Ray{direction: direction, side: side});
            }
        }
        let ray_count = rays.len();
        if ray_count > max_ray_count {
            max_ray_count = ray_count;
            best_asteroid = *asteroid;
            eprintln!("candidate: {} {} {}", asteroid.0, asteroid.1, max_ray_count);
        }
//        eprintln!("{} {} {} {:?}", asteroid.0, asteroid.1, ray_count, rays);
    }
    println!("{} {:?}", max_ray_count, best_asteroid);
    println!("{}", INFINITY.atan().to_degrees());
}