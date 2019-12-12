extern crate AoC2019;

use std::env;
use std::fs;
use std::fmt;
use std::collections::HashSet;

use AoC2019::{IntCode, run_intcode};

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Direction::Up {
            write!(f, "{}", '^')
        } else if *self == Direction::Right {
            write!(f, "{}", '>')
        } else if *self == Direction::Down {
            write!(f, "{}", 'v')
        } else if *self == Direction::Left {
            write!(f, "{}", '<')
        } else {
            write!(f, "O")
        }
    }
}

struct Canvas {
    width: usize,
    cells: Vec<u8>,
    position: usize,
    direction: Direction,
    painted_panels: HashSet<usize>
}

impl Canvas {
    fn x_y_to_pos(&self, x: usize, y: usize) -> usize {
        y*self.width + x
    }

    fn pos_to_x_y(&self) -> (usize, usize) {
        let y = self.position / self.width;
        let x = self.position - (y*self.width);
        (x, y)
    }

    fn with_width_height(width: usize, height: usize) -> Canvas {
        let midpoint_x = width / 2;
        let midpoint_y = height / 2;
        Canvas {
            width,
            cells: vec![0; width*height],
            position: midpoint_y*width + midpoint_x,
            direction: Direction::Up,
            painted_panels: HashSet::<usize>::new()
        }
    }

    fn turn_left_and_step(&mut self) {
        let coordinationes = self.pos_to_x_y();
        let mut x = coordinationes.0;
        let mut y = coordinationes.1;

        if self.direction == Direction::Up  {
            x -= 1;
            self.direction = Direction::Left;
        } else if self.direction == Direction::Left {
            y += 1;
            self.direction = Direction::Down;
        } else if self.direction == Direction::Down {
            x += 1;
            self.direction = Direction::Right;
        } else if self.direction == Direction::Right {
            y -= 1;
            self.direction = Direction::Up;
        }
        self.position = self.x_y_to_pos(x, y);
    }

    fn turn_right_and_step(&mut self) {
        let coordinationes = self.pos_to_x_y();
        let mut x = coordinationes.0;
        let mut y = coordinationes.1;

        if self.direction == Direction::Up  {
            x += 1;
            self.direction = Direction::Right;
        } else if self.direction == Direction::Right {
            y += 1;
            self.direction = Direction::Down;
        } else if self.direction == Direction::Down {
            x -= 1;
            self.direction = Direction::Left;
        } else if self.direction == Direction::Left {
            y -= 1;
            self.direction = Direction::Up;
        }
        self.position = self.x_y_to_pos(x, y);
    }

    fn get_colour(&self) -> u8 {
        self.cells[self.position]
    }

    fn set_colour(&mut self, colour: u8) {
        self.cells[self.position] = colour;
        self.painted_panels.insert(self.position);
    }

    fn draw(&self) {
        let height = self.cells.len() / self.width;
        for i in 0..height {
            for j in 0..self.width {
                if self.position == (i*height + j) {
                    print!("{}", self.direction);
                } else if self.cells[i*height + j] == 0 {
                    print!(" ");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let line = fs::read_to_string(filename).unwrap();
    let mut memory: Vec<i64> = line[0..(line.len()-1)].split(',').map(|el| { el.parse().unwrap() }).collect();
    let memory_extension_size = 100*1000;
    let mut memory_extension = vec![0; memory_extension_size];
    memory.append(&mut memory_extension);

    let mut machine = IntCode::new();
    machine.memory = memory;

    let width = 171;
    let height = 171;

    let mut canvas = Canvas::with_width_height(width, height);
    //sset the colour of the starting point
    canvas.set_colour(1);

    let mut step_count = 0;
    loop {
        let old_colour = canvas.get_colour() as i64;
        machine.input.push(old_colour);
        let new_colour = run_intcode(&mut machine);
        canvas.set_colour(new_colour as u8);
        let turn_direction = run_intcode(&mut machine);
//        eprintln!(">{}< {} :{}:", old_colour, new_colour, turn_direction);
        if turn_direction == 0 {
            canvas.turn_left_and_step();
        } else {
            canvas.turn_right_and_step();
        }

//        canvas.draw();
        step_count += 1;
        if machine.halted {
            break;
        }
    }
    canvas.draw();
    println!("{}", canvas.painted_panels.len());
//    eprintln!   ("{:?}", canvas.painted_panels);
}