extern crate petgraph;

use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

use petgraph::graphmap::UnGraphMap;

// visited contains the distances to all visited nodes
// distances contains the distances to all unvisited nodes
fn shortest_path(current_node: u32,
                 destination_node: u32, node_distance: u32,
                 graph: UnGraphMap<u32, u32>,
                 visited: &mut HashMap<u32, u32>,
                 distances: &mut HashMap<u32, u32>) {
    for node_id in graph.neighbors(current_node) {
        if !visited.contains_key(&node_id) {
            let tentative_distance = node_distance + 1;
            let current_distance = distances.get(&node_id).unwrap();
            if tentative_distance < *current_distance {
                distances.insert(node_id, tentative_distance);
            }
        }
    }
    visited.insert(current_node, node_distance);
    distances.remove(&current_node);
    if current_node == destination_node {
        eprintln!("found the destination");
        return
    } else {
        let mut shortest_path_to_neighbour = u32::max_value();
        let mut closest_neighbour = 0;
        for node_id in distances.keys() {
            let tentative_distance = distances.get(node_id).unwrap();
            if  *tentative_distance < shortest_path_to_neighbour {
                shortest_path_to_neighbour = *tentative_distance;
                closest_neighbour = *node_id;
            }
        }
        if distances.len() > 0 {
            shortest_path(closest_neighbour,
                          destination_node,
                          shortest_path_to_neighbour, graph,
                          visited, distances);
        } else {
            return
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut orbit_graph = UnGraphMap::<u32, u32>::new();
    let mut nodes = HashMap::<String, u32>::new();
    let mut node_num = 0;
    for line_maybe in reader.lines() {
        let line = line_maybe.unwrap();
        let mut objects = Vec::<u32>::new();
        for part in line.split(')') {
            let id = match nodes.get(&part.to_string()) {
                Some(val) => *val,
                None => {
                    let val = orbit_graph.add_node(node_num);
                    node_num += 1;
                    nodes.insert(part.to_string(), val);
                    val
                }
            };
            objects.push(id);
        }
        let obj1 = objects[0];
        let obj2 = objects[1];
        orbit_graph.add_edge(obj1, obj2, 1);
    }
    let start_node = nodes.get("YOU").unwrap();
    let end_node = nodes.get("SAN").unwrap();
    let mut visited = HashMap::<u32, u32>::with_capacity(orbit_graph.node_count());
    visited.insert(*start_node, 0);
    let mut distances = HashMap::<u32, u32>::with_capacity(orbit_graph.node_count());
    for node_id in 0..orbit_graph.node_count() {
        if node_id != *start_node as usize {
            distances.insert(node_id as u32, u32::max_value());
        }
    }
    eprintln!("find the shortest path");
    shortest_path(*start_node,
                  *end_node,
                  0,
                  orbit_graph,
                  &mut visited,
                  &mut distances
    );
    let path_length = visited.get(end_node).unwrap() - 2;
    println!("{}", path_length);
}