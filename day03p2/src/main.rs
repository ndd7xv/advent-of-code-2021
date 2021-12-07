use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let diagnostic = BufReader::new(File::open("input").unwrap())
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<String>>()
        .into_iter();

    let line_count = 1000;

    let zeroes = diagnostic
        .clone()
        .filter(|s| s.starts_with('0'))
        .collect::<Vec<String>>();
    let ones = diagnostic
        .filter(|s| s.starts_with('1'))
        .collect::<Vec<String>>();

    let mut least_common: Vec<String>;
    let mut most_common: Vec<String>;

    match zeroes.len().cmp(&(line_count / 2)) {
        // If zeroes are minotirty, then this is for co2 calculations and ones are o2 calculations
        Ordering::Less | Ordering::Equal => {
            least_common = zeroes;
            most_common = ones;
        }
        Ordering::Greater => {
            least_common = ones;
            most_common = zeroes;
        }
    }

    // Assumes we will find a valid rating
    // Does this for both oxygen and co2, might've been better to have a method that takes an Ordering criteria to reduce redundancy
    let mut index = 1;
    while least_common.len() != 1 {
        let mut zero_count = 0.0;
        let mut line_count = 0.0;
        least_common.clone().into_iter().for_each(|f| {
            if &f[index..index + 1] == "0" {
                zero_count += 1.0;
            }
            line_count += 1.0;
        });
        if zero_count <= line_count / 2.0 {
            least_common = least_common
                .into_iter()
                .filter(|s| &s[index..index + 1] == "0")
                .collect();
        } else {
            least_common = least_common
                .into_iter()
                .filter(|s| &s[index..index + 1] == "1")
                .collect();
        }
        index += 1;
    }

    let mut index = 1;
    while most_common.len() != 1 {
        let mut zero_count: f32 = 0.0;
        let mut line_count: f32 = 0.0;

        most_common.clone().into_iter().for_each(|f| {
            println!("{}", f);
            if &f[index..index + 1] == "0" {
                zero_count += 1.0;
            }
            line_count += 1.0;
        });

        if zero_count <= line_count / 2.0 {
            most_common = most_common
                .into_iter()
                .filter(|s| &s[index..index + 1] == "1")
                .collect();
        } else {
            most_common = most_common
                .into_iter()
                .filter(|s| &s[index..index + 1] == "0")
                .collect();
        }

        index += 1;
    }

    println!(
        "{} and {}",
        isize::from_str_radix(&most_common[0], 2).unwrap(),
        isize::from_str_radix(&least_common[0], 2).unwrap()
    );
}
