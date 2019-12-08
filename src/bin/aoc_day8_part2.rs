use std::env;
use std::fs;

fn print_layer(layer: &Vec<u8>, width: usize, height: usize) {
    for i in 0..height {
        for j in 0..width {
            let colour = layer[i*width + j];
            let char = if colour == 2 || colour == 0 { ' ' } else { '*' };
            print!("{}", char);
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
    let mut layer_num = 0;
    let mut final_image: Vec<u8> = vec![2; img_size];
    loop {
        for i in 0..img_width {
            for j in 0..img_height {
                let colour = line[layer_num*img_size + j*img_width + i] - 48;
                if colour != 2 && final_image[j*img_width + i] == 2 {
                    final_image[j*img_width + i] = colour;
                }
            }
        }
        layer_num += 1;
        if layer_num*img_size >= (line.len()-1) {
            break;
        }
    }
    print_layer(&final_image, img_width, img_height);
}