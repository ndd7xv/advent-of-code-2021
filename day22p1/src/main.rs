use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    ops::RangeInclusive,
};

struct Instruction {
    turn_on: bool,
    x_range: RangeInclusive<i32>,
    z_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

fn main() {
    let instructions = parse_input("inputs/input");

    println!("CUBES ON: {}", initialize(instructions));
}

fn initialize(instructions: Vec<Instruction>) -> usize {
    let mut points = HashSet::new();

    for instruction in instructions {
        if -50 <= *instruction.x_range.start()
            && *instruction.x_range.end() <= 50
            && -50 <= *instruction.y_range.start()
            && *instruction.y_range.end() <= 50
            && -50 <= *instruction.z_range.start()
            && *instruction.z_range.end() <= 50
        {
            if instruction.turn_on {
                for x in instruction.x_range {
                    for y in instruction.y_range.clone() {
                        for z in instruction.z_range.clone() {
                            points.insert((x, y, z));
                        }
                    }
                }
            } else {
                for x in instruction.x_range {
                    for y in instruction.y_range.clone() {
                        for z in instruction.z_range.clone() {
                            points.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }

    points.len()
}

fn parse_input(file: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    io::BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .for_each(|s| {
            let s = s.split(' ').collect::<Vec<&str>>();

            let turn_on = match s[0] {
                "on" => true,
                "off" => false,
                _ => {
                    panic!("Input is malformed. ");
                }
            };

            let ranges = s[1].split(',').collect::<Vec<&str>>();

            let mut x_range = 0..=0;
            let mut y_range = 0..=0;
            let mut z_range = 0..=0;

            for range in ranges {
                let bounds = range[2..].split("..").collect::<Vec<&str>>();
                if range.starts_with("x=") {
                    x_range = RangeInclusive::new(
                        bounds[0].parse::<i32>().unwrap(),
                        bounds[1].parse::<i32>().unwrap(),
                    );
                } else if range.starts_with("y=") {
                    y_range = RangeInclusive::new(
                        bounds[0].parse::<i32>().unwrap(),
                        bounds[1].parse::<i32>().unwrap(),
                    );
                } else if range.starts_with("z=") {
                    z_range = RangeInclusive::new(
                        bounds[0].parse::<i32>().unwrap(),
                        bounds[1].parse::<i32>().unwrap(),
                    );
                }
            }

            instructions.push(Instruction {
                turn_on,
                x_range,
                y_range,
                z_range,
            });
        });

    instructions
}

#[test]
fn small_initialization_example() {
    let instructions = parse_input("inputs/small_initialization");
    assert_eq!(39, initialize(instructions));
}

#[test]
fn medium_initialization_example() {
    let instructions = parse_input("inputs/medium");
    assert_eq!(590784, initialize(instructions));
}
