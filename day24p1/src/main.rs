use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl ALU {
    fn new() -> Self {
        ALU {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
    fn run(&mut self, program: &[Vec<String>], input: &str) {
        let mut input = input.chars();

        for instruction in program {
            match &instruction[0][..] {
                "inp" => self.inp(&instruction[1][..], &input.next().unwrap().to_string()[..]),
                "add" => self.add(&instruction[1][..], &instruction[2][..]),
                "mul" => self.mul(&instruction[1][..], &instruction[2][..]),
                "div" => self.div(&instruction[1][..], &instruction[2][..]),
                "mod" => self.modulo(&instruction[1][..], &instruction[2][..]),
                "eql" => self.eql(&instruction[1][..], &instruction[2][..]),
                _ => {
                    println!("Unrecognized instruction: {}", instruction[0])
                }
            }
        }
    }
    fn match_var(&mut self, a: &str) -> Option<&mut i64> {
        match a {
            "w" => Some(&mut self.w),
            "x" => Some(&mut self.x),
            "y" => Some(&mut self.y),
            "z" => Some(&mut self.z),
            _ => None,
        }
    }
    fn inp(&mut self, a: &str, val: &str) {
        let a = self.match_var(a).unwrap();

        *a = val.parse::<i64>().unwrap();
    }
    fn add(&mut self, a: &str, b: &str) {
        let b = *self
            .match_var(b)
            .unwrap_or(&mut b.parse::<i64>().unwrap_or_default());
        let a = self.match_var(a).unwrap();

        *a += b;
    }
    fn mul(&mut self, a: &str, b: &str) {
        let b = *self
            .match_var(b)
            .unwrap_or(&mut b.parse::<i64>().unwrap_or_default());
        let a = self.match_var(a).unwrap();

        *a *= b;
    }
    fn div(&mut self, a: &str, b: &str) {
        let b = *self
            .match_var(b)
            .unwrap_or(&mut b.parse::<i64>().unwrap_or_default());
        let a = self.match_var(a).unwrap();

        *a /= b;
    }
    fn modulo(&mut self, a: &str, b: &str) {
        let b = *self
            .match_var(b)
            .unwrap_or(&mut b.parse::<i64>().unwrap_or_default());
        let a = self.match_var(a).unwrap();

        *a %= b;
    }
    fn eql(&mut self, a: &str, b: &str) {
        let b = *self
            .match_var(b)
            .unwrap_or(&mut b.parse::<i64>().unwrap_or_default());
        let a = self.match_var(a).unwrap();

        if *a == b {
            *a = 1
        } else {
            *a = 0
        };
    }
}

fn main() {
    let program = parse_input("inputs/input");

    let mut largest: i64 = 99999999999999;
    let mut alu = ALU::new();

    // 999819949?
    // 99692994994

    // Trying 98692994993
    // Stuck at input 12

    // WORKS: 58692994993998
    // WORKS: 59692994994998

    loop {
        if !has_no_zeroes(largest) {
            largest += 1;
            continue;
        }

        alu.run(&program, &largest.to_string()[..]);
        largest -= 1;

        if alu.z == 0 {
            break;
        }

        alu = ALU::new();
    }
    println!("{}: {:?}", largest, alu);
}

fn has_no_zeroes(num: i64) -> bool {
    let mut num = num;
    loop {
        if num % 10 == 0 {
            return false;
        }
        num /= 10;
        if num < 10 {
            return true;
        }
    }
}

fn parse_input(path: &str) -> Vec<Vec<String>> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split(' ').map(String::from).collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}
