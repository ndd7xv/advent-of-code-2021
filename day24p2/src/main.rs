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
    fn run(&mut self, program: &Vec<Vec<String>>, input: &str) {
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
    
    let largest: i64 = 16181111641521;
    let mut alu = ALU::new();

    alu.run(&program, &largest.to_string()[..]);

    println!("{}: {:?}", largest, alu);
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
