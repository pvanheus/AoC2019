use std::env;
use std::fs;

fn print_layer(layer: &Vec<u8>, width: usize, height: usize) {
    for i in 0..height {
        for j in 0..width {
            print!("{}", layer[i*width + j]);
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let line = fs::read_to_string(filename).unwrap().into_bytes();

    let img_width = 25;
    let img_height = 6;
    let img_size = img_width * img_height;
//    let mut layers = Vec::<Vec<u8>>::new();
    let mut min_zero_count = i32::max_value();
    let mut result = 0;
    let mut layer_num = 0;
    loop {
        let mut current_layer = Vec::<u8>::with_capacity(img_height * img_width);
        for i in 0..img_width {
            for j in 0..img_height {
                let colour = line[layer_num*img_size + j*img_width + i] - 48;
                current_layer.push(colour);
            }
//        layers.push(current_layer);
        }
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut two_count = 0;
        for colour in &current_layer {
            if *colour == 0 {
                zero_count += 1;
            } else if *colour == 1 {
                one_count += 1;
            } else if *colour == 2 {
                two_count += 1;
            }
        }
        if zero_count < min_zero_count {
            eprintln!("choosing: {}", layer_num);
            min_zero_count = zero_count;
            result = one_count * two_count;
            print_layer(&current_layer, img_width, img_height);
        }
        layer_num += 1;
        if layer_num*img_size >= (line.len()-1) {
            break;
        }
    }
    eprintln!("end: {}", layer_num);
    println!("{}", result);
}