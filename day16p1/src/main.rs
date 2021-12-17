use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let binary = parse_input("inputs/input");
    let result = interpret(&binary[..]);

    println!("VERSION SUMS:{}", result.0);
}

fn interpret(binary: &str) -> (u32, u32) {
    let version = str_to_u32(&binary[0..3]);
    let type_id = str_to_u32(&binary[3..6]);

    let mut version_count = version;

    if type_id == 4 {
        let mut value = String::new();
        let mut curr_bits = 0;
        let mut start = 6 + curr_bits;
        let mut end = 7 + curr_bits;
        while str_to_u32(&binary[start..end]) != 0 {
            value.push_str(&binary[start + 1..start + 5]);
            curr_bits += 5;
            start = 6 + curr_bits;
            end = 7 + curr_bits;
        }
        value.push_str(&binary[start + 1..start + 5]);

        (version_count, (6 + curr_bits + 5) as u32)
    } else {
        let length_type_id = str_to_u32(&binary[6..7]);
        if length_type_id == 0 {
            let total_bits = str_to_u32(&binary[7..22]);
            let mut curr_bits = 0;
            while curr_bits < total_bits {
                let start = 22 + curr_bits as usize;
                let sub_calculation = interpret(&binary[start..]);
                version_count += sub_calculation.0;
                curr_bits += sub_calculation.1;
            }
            (version_count, 22 + total_bits)
        } else {
            let subpacket_num = str_to_u32(&binary[7..18]);
            let mut curr_bits = 0;
            for _i in 0..subpacket_num {
                let start = 18 + curr_bits as usize;
                let sub_calculation = interpret(&binary[start..]);
                version_count += sub_calculation.0;
                curr_bits += sub_calculation.1;
            }

            (version_count, (18 + curr_bits) as u32)
        }
    }
}

fn str_to_u32(str: &str) -> u32 {
    u32::from_str_radix(str, 2).unwrap()
}

fn parse_input(file: &str) -> String {
    fn to_hex(c: char) -> String {
        match c {
            '0' => String::from("0000"),
            '1' => String::from("0001"),
            '2' => String::from("0010"),
            '3' => String::from("0011"),
            '4' => String::from("0100"),
            '5' => String::from("0101"),
            '6' => String::from("0110"),
            '7' => String::from("0111"),
            '8' => String::from("1000"),
            '9' => String::from("1001"),
            'A' => String::from("1010"),
            'B' => String::from("1011"),
            'C' => String::from("1100"),
            'D' => String::from("1101"),
            'E' => String::from("1110"),
            'F' => String::from("1111"),
            _ => String::from("ERROR"),
        }
    }
    io::BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|r| {
            r.unwrap()
                .chars()
                .map(to_hex)
                .collect::<Vec<String>>()
                .join("")
        })
        .next()
        .unwrap()
}

#[test]
fn test_case_1() {
    let binary = parse_input("inputs/test1");
    let result = interpret(&binary[..]);
    assert_eq!(result.0, 16);
}

#[test]
fn test_case_2() {
    let binary = parse_input("inputs/test2");
    let result = interpret(&binary[..]);
    assert_eq!(result.0, 12);
}

#[test]
fn test_case_3() {
    let binary = parse_input("inputs/test3");
    let result = interpret(&binary[..]);
    assert_eq!(result.0, 23);
}

#[test]
fn test_case_4() {
    let binary = parse_input("inputs/test4");
    let result = interpret(&binary[..]);
    assert_eq!(result.0, 31);
}

#[test]
fn basic() {
    let binary = parse_input("inputs/basic");
    let result = interpret(&binary[..]);
    assert_eq!(result.0, 6);
}

#[test]
fn basic_2() {
    let binary = parse_input("inputs/basic2");
    let result = interpret(&binary[..]);
    assert_eq!(result.0, 9);
}

#[test]
fn basic_3() {
    let binary = parse_input("inputs/basic3");
    let result = interpret(&binary[..]);
    assert_eq!(result.0, 14);
}

#[test]
fn custom_case() {
    let binary = parse_input("inputs/custom_case");
    let result = interpret(&binary[..]);
    assert_eq!(result.0, 6);
}
