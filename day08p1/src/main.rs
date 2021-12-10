use std::fs;

fn main() {
    let inputs = fs::read_to_string("test").unwrap();

    let outputs = inputs
        .lines()
        .map(|entry| entry.split(" | ").last().unwrap());

    let easy_digits: usize = outputs
        .map(|output| {
            output
                .split(' ')
                .filter(|digit| {
                    let char_count = digit.chars().count();
                    char_count == 2 || char_count == 3 || char_count == 4 || char_count == 7
                })
                .count()
        })
        .sum();

    println!("{}", easy_digits);
}
