use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let mut solution = calculate_fuel(&input, *input.iter().max().unwrap());

    for x_pos in *input.iter().min().unwrap()..*input.iter().max().unwrap() {
        let contender = calculate_fuel(&input, x_pos);
        if contender < solution {
            solution = contender;
        }
    }

    println!("{}", solution);
}

fn calculate_fuel(input: &[i32], x_pos: i32) -> i32 {
    let mut fuel = 0;

    for i in input {
        fuel += (x_pos - i).abs();
    }

    fuel
}
