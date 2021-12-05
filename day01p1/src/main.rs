use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input").unwrap();
    let lines: Vec<i32> = io::BufReader::new(f)
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut count = 0;
    for (current, next) in lines.iter().zip(lines.iter().skip(1)) {
        if current < next {
            count += 1;
        }
    }

    println!("{}", count);
}