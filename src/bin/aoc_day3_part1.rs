use std::cmp;
use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn print_map(map: &HashMap<(i32, i32), u32>) {
    let mut min_x = i32::max_value();
    let mut min_y = i32::max_value();
    let mut max_x = i32::min_value();
    let mut max_y = i32::min_value();
    for key in map.keys() {
        min_x = cmp::min(min_x, key.0);
        min_y = cmp::min(min_y, key.1);
        max_x = cmp::max(max_x, key.0);
        max_y = cmp::max(max_y, key.1);
    }
    for x in ((min_x - 1)..=(max_x+1)).rev() {
        for y in (min_y - 1)..=(max_y+1) {
            if x == 0 && y == 0 {
                eprint!("O");
            } else {
                let content = map.get(&(x, y));
                match content {
                    Some(val) => {
                        if val.count_ones() > 1 {
                            eprint!("X");
                        } else {
                            eprint!("{}", (*val as f64).log2());
                        }
                    },
                    None => {
                        eprint!(".");
                    }
                }
            }
        }
        eprintln!();
    }
}

fn check_point(x: i32, y: i32, id: u32, map: &HashMap<(i32, i32), u32>) -> i32 {
    let here = (x, y);
    if map.contains_key(&here) && (map.get(&here).unwrap() & 2_u32.pow(id) == 0) {
        let dist = x.abs() + y.abs();
//        eprintln!("found an overlap {}", dist);
        return dist;
    } else {
        return i32::max_value();
    }
}

fn trace_path(path: &str, id: u32, map: &mut HashMap<(i32, i32), u32>) -> i32 {
    let mut pos = (0, 0);
    let mut distance = i32::max_value();
    for mv_str in path.split(',') {
        let mv = mv_str.as_bytes();
//        eprintln!("{}", std::str::from_utf8(&mv[1..mv.len()]).unwrap());
        let moved_distance = std::str::from_utf8(&mv[1..mv.len()]).unwrap().parse::<i32>().unwrap();
        let direction = mv[0];
        for _ in 1..=moved_distance {
            match direction {
                b'L' => pos.0 -= 1,
                b'R' => pos.0 += 1,
                b'D' => pos.1 -= 1,
                b'U' => pos.1 += 1,
                _ => panic!("unknown directive encountered")
            }
            distance = cmp::min(check_point(pos.0, pos.1, id, map), distance);
            let marker = match map.get(&pos) {
                Some(val) => val | 2_u32.pow(id),
                None => 2_u32.pow(id)
            };
            map.insert(pos, marker);
        }
    }
    return distance
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let in_file = File::open(filename).unwrap();
    let reader = BufReader::new(in_file);
    let mut distance = i32::max_value();
    let mut id = 1;
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        distance = cmp::min(trace_path(&line, id, &mut map), distance);
        id += 1;
    }
//    print_map(&map);
    println!("{}", distance)
}