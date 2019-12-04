use std::cmp;
use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn check_point(x: i32, y: i32, id: i32, path_length: i32, map: &HashMap<(i32, i32), HashMap<i32, i32>>) -> i32 {
    let here = (x, y);
    let mut dist = i32::max_value();
    if map.contains_key(&here) {
        for (other_id, other_path_length) in map.get(&here).unwrap().iter() {
            if *other_id == id {
                continue
            }
            dist = cmp::min(other_path_length + path_length, dist);
        }
        return dist;
    } else {
        return i32::max_value();
    }
}

fn trace_path(path: &str, id: i32, map: &mut HashMap<(i32, i32), HashMap<i32, i32>>) -> i32 {
    let mut pos = (0, 0);
    let mut distance = i32::max_value();
    let mut path_length = 0;
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
            path_length += 1;
            distance = cmp::min(check_point(pos.0, pos.1, id, path_length, map), distance);
            if map.contains_key(&pos) {
                let val = map.get_mut(&pos).unwrap();
                if !val.contains_key(&id) {
                        val.insert(id, path_length);
                    }
            } else {
                let mut val: HashMap<i32, i32> = HashMap::new();
                val.insert(id, path_length);
                map.insert(pos, val);
            }
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
    let mut id = 1_i32;
    let mut map: HashMap<(i32, i32), HashMap<i32, i32>> = HashMap::new();
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        distance = cmp::min(trace_path(&line, id, &mut map), distance);
        id += 1;
    }
//    print_map(&map);
    println!("{}", distance)
}