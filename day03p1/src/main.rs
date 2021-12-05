use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let diagnostic = BufReader::new(File::open("input").unwrap()).lines();
    // Considerations:
    //  - How to idiomatically and dynamically get the size of the array? A lazy Option<Vec<u32>> initializer did not work
    //  - How to get count more idiomatically? Or is this sufficable?
    let bits: u32 = 12;

    let mut totals = vec![0; bits as usize];
    let mut line_count = 0;

    for binary in diagnostic.flatten() {
        for (index, num) in binary.chars().map(|c| c.to_digit(2).unwrap()).enumerate() {
            totals[index] += num;
        }
        line_count += 1;
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    totals.iter().enumerate().for_each(|(index, count)| {
        if *count > line_count / 2 {
            gamma += 2_i32.pow((bits - 1) - index as u32);
        } else {
            epsilon += 2_i32.pow((bits - 1) - index as u32);
        }
    });

    println!("{}", gamma * epsilon);
}