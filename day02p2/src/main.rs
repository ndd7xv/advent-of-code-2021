use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut depth = 0;
    let mut horizontal_position = 0;
    let mut aim = 0;
    
    // Operates under the assumption the submarine cannot have negative depth
    let course = BufReader::new(File::open("input").unwrap()).lines();

    for instruction in course.flatten() {
        if let [direction, x] = instruction.split(' ').collect::<Vec<&str>>()[..] {
            let x = x.parse::<i32>().unwrap();
            match direction {
                "forward" => { 
                    horizontal_position += x; 
                    depth += aim * x;
                },
                "up" => aim -= x,
                "down" => aim += x,
                _ => {}
            }
        }
    }

    println!("{}", horizontal_position * depth)
}
