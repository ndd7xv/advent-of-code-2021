use std::fs::File;
use std::io::{self, BufRead};
use std::iter;

fn main() {
    let f = File::open("input").unwrap();
    let lines: Vec<i32> = io::BufReader::new(f)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut count = 0;
    let mut tracker: Option<i32> = None;

    for ((a, b), c) in lines
        .iter()
        .zip(lines.iter().skip(1).chain(iter::repeat(&0)))
        .zip(lines.iter().skip(2).chain(iter::repeat(&0)))
    {
        let current = a + b + c;

        if let Some(previous) = tracker {
            if previous < current {
                count += 1
            }
        }

        tracker = Some(current);
    }

    println!("{}", count);
}