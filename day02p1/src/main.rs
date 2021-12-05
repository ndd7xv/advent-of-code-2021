use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut depth = 0;
    let mut horizontal_position = 0;

    // Operates under the assumption the submarine cannot have negative depth
    let course = BufReader::new(File::open("input").unwrap()).lines();

    for instruction in course.flatten() {
        if let [direction, distance] = instruction.split(' ').collect::<Vec<&str>>()[..] {
            let distance = distance.parse::<i32>().unwrap();
            match direction {
                "forward" => horizontal_position += distance,
                "up" => depth -= distance,
                "down" => depth += distance,
                _ => {}
            }
        }
    }

    println!("{}", horizontal_position * depth)
}