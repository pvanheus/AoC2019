use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap, BinaryHeap};
use std::f64::INFINITY;

#[derive(PartialEq)]
#[derive(Debug)]
struct Ray {
    direction: f64,
    side: i32,
    distance: i32
}

impl Hash for Ray {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.direction.to_string().hash(state);
        self.side.hash(state);
        self.distance.hash(state);
    }
}

impl Eq for Ray {}

impl Ord for Ray {
    fn cmp(&self, other: &Self) -> Ordering {
        // rays start at side 1, direction 0
        if self.direction == other.direction {
            self.distance.cmp(&other.distance)
        } else {
            match self.direction.partial_cmp(&other.direction) {
                Some(ord) => ord,
                None => {
                    eprintln!("warning: no ordering for {:?} and {:?}", self, other);
                    Ordering::Equal
                }
            }
        }
    }
}

impl PartialOrd for Ray {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(Debug)]
#[derive(Copy,Clone)]
struct Asteroid {
    x: i32,
    y: i32
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Copy,Clone)]
#[derive(Debug)]
struct Target {
    asteroid: Asteroid,
    distance: i32
}

impl Ord for Target {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn get_direction(asteroid: &Asteroid, other_asteroid: &Asteroid) -> f64 {
    let d_x = other_asteroid.x - asteroid.x;
    let d_y = asteroid.y - other_asteroid.y;
    if d_x == 0 {
        // straight up or straight down
        if d_y < 0 {
            // up
            180.0
        } else {
            // down
            0.0
        }
    } else if d_x > 0 && d_y >= 0 {
        (d_x as f64 / d_y as f64).atan().to_degrees()
    } else if d_x > 0 && d_y < 0 {
        90.0 + -(d_y as f64 / d_x as f64).atan().to_degrees()
    } else if d_x < 0 && d_y < 0 {
        180.0 + (d_x as f64 / d_y as f64).atan().to_degrees()
    } else if d_x < 0 && d_y >= 0 {
        270.0 + -(d_y as f64 / d_x as f64).atan().to_degrees()
    } else {
        // should never get here
        -1.0
    }
}

fn find_best_astroid(asteroids: &Vec<Asteroid>) -> &Asteroid {
    let mut max_ray_count = 0;
    let mut best_asteroid= &Asteroid { x: 0, y: 0 };
    for asteroid in asteroids {
        let mut rays = HashSet::<i32>::new();
        for other_asteroid in asteroids {
            if !(asteroid == other_asteroid) {
                let direction = get_direction(asteroid, other_asteroid) * 10000.0;
                rays.insert(direction as i32);
            }
        }
        let ray_count = rays.len();
        if ray_count > max_ray_count {
            max_ray_count = ray_count;
//            eprintln!("candidate: {} {} {}", max_ray_count, asteroid.x, asteroid.y);
            best_asteroid = asteroid;
        }
    }
    best_asteroid
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut asteroids = Vec::<Asteroid>::new();
    let mut y = 0;
    for line_maybe in reader.lines() {
        let line = line_maybe.unwrap();
        let mut x = 0;
        for char in line.chars() {
            if char == '.' || char == '#' {
                if char == '#' {
                    asteroids.push(Asteroid { x: x, y: y });
                }
            }
            x += 1;
        }
        y += 1;
    }

    eprintln!("total asteroids: {}", asteroids.len());
    let asteroid = find_best_astroid(&asteroids);
    eprintln!("station at: {} {}", asteroid.x, asteroid.y);
    let mut targets = HashMap::<i32, BinaryHeap<Target>>::with_capacity(asteroids.len());
    for other_asteroid in &asteroids {
        if *other_asteroid != *asteroid {
            let distance = (other_asteroid.y - asteroid.y).pow(2) + (other_asteroid.x - asteroid.x).pow(2); // squared distance
            let direction = (get_direction(&asteroid, &other_asteroid) * 10000.0) as i32;
//            let tan = if (other_asteroid.0 - asteroid.0) == 0 { INFINITY } else { ((other_asteroid.1 - asteroid.1) as f64 / (other_asteroid.0 - asteroid.0) as f64) };
//            let direction = tan.atan().to_degrees() + 90.0;
            targets.entry(direction).or_insert(BinaryHeap::<Target>::new()).push(Target { asteroid: other_asteroid.clone(), distance });
        }
    }
//    let mut targets_vector = Vec::<Asteroid>::with_capacity(asteroids.len());
    let mut directions: Vec<i32> = targets.keys().map(|x| *x).collect();
    directions.sort();

    let mut sorted_targets = HashMap::<i32, Vec<&Target>>::new();
    let mut directions_len = 0;
    for direction in &directions {
        directions_len += 1;
        let mut target_list = Vec::<&Target>::new();
        for target in targets.get(&direction).unwrap() {
            target_list.insert(0, target);
        }
        sorted_targets.insert(*direction, target_list);
    };
    let mut zap_count = 0;
    let mut direction_index = 0;
    let mut current_target = Target { asteroid: Asteroid { x: 0, y: 0}, distance: 0};
    eprintln!("LEN: {}", directions_len);
    loop {
        if direction_index >= directions_len {
            direction_index = 0;
        }
        let direction = directions[direction_index];
        if sorted_targets.contains_key(&direction) {
            let target_list = sorted_targets.get(&direction).unwrap();
            if target_list.len() > 0 {
                current_target = target_list[0].clone();
//                eprintln!("{} {}", current_target.asteroid.x, current_target.asteroid.y);
                let mut new_target_list = target_list.clone();
                new_target_list.remove(0);
                sorted_targets.insert(direction, new_target_list);
                zap_count += 1;
            }
        }
        direction_index += 1;
        if zap_count == 200 {
            eprintln!("200th zap: {} {} {}", current_target.asteroid.x, current_target.asteroid.y, current_target.asteroid.x * 100 + current_target.asteroid.y);
        }
        if zap_count == asteroids.len() - 1 {
            break
        }
    }
//    eprintln!("{:?}", sorted_targets);
}