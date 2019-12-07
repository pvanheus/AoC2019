use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};

fn count_orbits(start: Option<&HashSet<String>>, depth: i32, orbits: &HashMap<String, HashSet<String>>) -> i32 {
    let mut orbit_count = 0;
    orbit_count = match start {
        Some(objects) => {
            for object in objects.iter() {
                orbit_count += depth + count_orbits(orbits.get(object), depth+1, orbits);
            }
            orbit_count
        },
        None => orbit_count
    };
    orbit_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut orbits: HashMap<String, HashSet<String>> = HashMap::new();
    for line_maybe in reader.lines() {
        let line = line_maybe.unwrap();
        let mut objects: Vec<String> = Vec::new();
        for part in line.split(')') {
            objects.push(part.to_string());
        }
        let obj1 = &objects[0];
        let obj2 = &objects[1];
        match orbits.get_mut(obj1) {
            Some(o) => { o.insert(obj2.clone()); () },
            None => {
                let mut o: HashSet<String> = HashSet::with_capacity(1);
                o.insert(obj2.clone());
                orbits.insert(obj1.clone(), o);
                ()
            }
        }
    }
    let orbit_count = count_orbits(orbits.get("COM"), 1, &orbits);
    println!("{}", orbit_count);
}