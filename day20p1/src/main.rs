use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

struct Image {
    pixels: HashMap<(i32, i32), bool>,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    lit: bool,
}

fn main() {
    let (enhancement_algo, mut image) = parse_input("inputs/input");
    for _i in 0..2 {
        enhance(&mut image, &enhancement_algo);
    }

    println!(
        "LIT PIXELS: {}",
        image.pixels.iter().filter(|(_k, v)| **v).count()
    );
}

fn enhance(image: &mut Image, enhancement_algo: &str) {
    let mut updated_pixels = HashMap::new();

    for row in image.y_bounds.0 - 1..=image.y_bounds.1 + 1 {
        for col in image.x_bounds.0 - 1..=image.x_bounds.1 + 1 {
            // Create the 3x3 string representation
            let mut binary = String::new();
            for i in row - 1..=row + 1 {
                for j in col - 1..=col + 1 {
                    binary.push(if *image.pixels.entry((i, j)).or_insert(image.lit) {
                        '1'
                    } else {
                        '0'
                    });
                }
            }
            let enhancement_code = usize::from_str_radix(&binary, 2)
                .expect("The binary representation should consist of 1 and 0");
            updated_pixels.insert(
                (row, col),
                &enhancement_algo[enhancement_code..enhancement_code + 1] == "#",
            );
        }
    }
    image.pixels = updated_pixels;

    image.x_bounds.0 -= 1;
    image.x_bounds.1 += 1;

    image.y_bounds.0 -= 1;
    image.y_bounds.1 += 1;

    if &enhancement_algo[0..1] == "#" {
        image.lit = !image.lit
    }
}

fn parse_input(file: &str) -> (String, Image) {
    let mut lines = io::BufReader::new(File::open(file).unwrap()).lines();

    // Input is assumed to always be correct - enhancement algorithm followed by an empty line
    let enhancement_algo = lines.next().unwrap().unwrap();
    lines.next();

    let mut pixels = HashMap::new();

    let mut x_bounds = (0, 0);
    let mut y_bounds = (0, 0);

    for (row, line) in lines.enumerate() {
        for (column, char) in line.unwrap().chars().enumerate() {
            if char == '#' {
                let row = row as i32;
                let column = column as i32;

                pixels.insert((row, column), true);

                if column < x_bounds.0 {
                    x_bounds.0 = column
                } else if column > x_bounds.1 {
                    x_bounds.1 = column
                }

                if row < y_bounds.0 {
                    y_bounds.0 = row
                } else if row > y_bounds.1 {
                    y_bounds.1 = row
                }
            }
        }
    }

    (
        enhancement_algo,
        Image {
            pixels,
            x_bounds,
            y_bounds,
            lit: false,
        },
    )
}

#[test]
fn example() {
    let (enhancement_algo, mut image) = parse_input("inputs/test");

    for _i in 0..2 {
        enhance(&mut image, &enhancement_algo);
    }

    assert_eq!(image.pixels.iter().filter(|(_k, v)| **v).count(), 35);
}
