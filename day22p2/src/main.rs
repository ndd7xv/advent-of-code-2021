use std::{
    cmp,
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

    println!("CUBES ON: {}", count_cubes(instructions));
}

fn count_cubes(instructions: Vec<Instruction>) -> i64 {
    let mut processed_instructions = vec![];
    let mut total_cubes = 0;

    // Go in reverse because this allows us to just calculate non-overlapping regions
    // instead of keeping track of deleted space for future cuboid ranges
    for instruction in instructions.into_iter().rev() {
        if instruction.turn_on {
            // If we're turning lights on, we need to account for the overlap in 'previous' cuboids
            total_cubes +=
                cubes(&instruction) - overlapping_cubes(&instruction, &processed_instructions);
        }
        // If not, we just add it - because if forward order started with an 'off', it wouldn't affect the running sum
        processed_instructions.push(instruction);
    }

    total_cubes
}

fn overlapping_cubes(instruction: &Instruction, processed_instructions: &[Instruction]) -> i64 {
    // Find the overlap with all of the existing instructions
    processed_instructions
        .iter()
        .enumerate()
        .map(|(index, inst)| {
            let overlap_x = (
                cmp::max(inst.x_range.start(), instruction.x_range.start()),
                cmp::min(inst.x_range.end(), instruction.x_range.end()),
            );
            let overlap_y = (
                cmp::max(inst.y_range.start(), instruction.y_range.start()),
                cmp::min(inst.y_range.end(), instruction.y_range.end()),
            );
            let overlap_z = (
                cmp::max(inst.z_range.start(), instruction.z_range.start()),
                cmp::min(inst.z_range.end(), instruction.z_range.end()),
            );

            // If overlap exists with a specific cube, we return the calculated overlap
            // however, we do not want to double count overlap, so we check for overlap of the overlap
            if overlap_x.1 - overlap_x.0 >= 0
                && overlap_y.1 - overlap_y.0 >= 0
                && overlap_z.1 - overlap_z.0 >= 0
            {
                let overlapping_inst = Instruction {
                    turn_on: true,
                    x_range: *overlap_x.0..=*overlap_x.1,
                    y_range: *overlap_y.0..=*overlap_y.1,
                    z_range: *overlap_z.0..=*overlap_z.1,
                };

                cubes(&overlapping_inst)
                    - overlapping_cubes(&overlapping_inst, &processed_instructions[index + 1..])
            } else {
                0
            }
        })
        .sum()
}

fn cubes(instruction: &Instruction) -> i64 {
    (instruction.x_range.end() - instruction.x_range.start() + 1) as i64
        * (instruction.y_range.end() - instruction.y_range.start() + 1) as i64
        * (instruction.z_range.end() - instruction.z_range.start() + 1) as i64
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
fn medium_initialization_example() {
    let instructions = parse_input("inputs/medium");
    assert_eq!(2758514936282235, count_cubes(instructions));
}

#[test]
fn cuboid_volume() {
    let inst = Instruction {
        turn_on: true,
        x_range: 10..=12,
        y_range: 10..=12,
        z_range: 10..=12,
    };
    assert_eq!(cubes(&inst), 27);
}
