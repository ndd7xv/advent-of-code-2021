use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
enum Direction {
    Horizontal(u32),
    Vertical(u32),
}

fn main() {
    let (mut coords, instructions) = parse_input("inputs/input");
    let coords = fold(&mut coords, instructions.get(0).unwrap());
    println!("{}", coords.len());
}

fn parse_input(file: &str) -> (HashSet<(u32, u32)>, Vec<Direction>) {
    let mut lines = io::BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|r| r.unwrap());

    let mut coords = HashSet::new();
    let mut instructions = vec![];

    for c in &mut lines {
        if c.is_empty() {
            break;
        }
        let pos = c.split(',').collect::<Vec<&str>>();
        coords.insert((
            pos[0].parse::<u32>().unwrap(),
            pos[1].parse::<u32>().unwrap(),
        ));
    }

    for instruction in lines {
        if instruction.contains("x=") {
            instructions.push(Direction::Vertical(
                instruction[instruction.chars().position(|c| c == '=').unwrap() + 1..]
                    .parse::<u32>()
                    .unwrap(),
            ));
        } else if instruction.contains("y=") {
            instructions.push(Direction::Horizontal(
                instruction[instruction.chars().position(|c| c == '=').unwrap() + 1..]
                    .parse::<u32>()
                    .unwrap(),
            ))
        }
    }

    (coords, instructions)
}

fn fold(coords: &mut HashSet<(u32, u32)>, direction: &Direction) -> HashSet<(u32, u32)> {
    let mut new_coords = HashSet::new();
    match direction {
        Direction::Horizontal(line) => {
            let t = coords.drain();
            for coord in t {
                if coord.1 > *line {
                    new_coords.insert((coord.0, 2 * line - coord.1));
                } else {
                    new_coords.insert(coord);
                }
            }
        }
        Direction::Vertical(line) => {
            let t = coords.drain();
            for coord in t {
                if coord.0 > *line {
                    new_coords.insert((2 * line - coord.0, coord.1));
                } else {
                    new_coords.insert(coord);
                }
            }
        }
    }
    new_coords
}

#[test]
fn test_case_1() {
    let (mut coords, instructions) = parse_input("inputs/test");
    let mut coords = fold(&mut coords, instructions.get(0).unwrap());
    let coords = fold(&mut coords, instructions.get(1).unwrap());
    println!("{:?}", coords);
    assert_eq!(17, coords.len());
}
