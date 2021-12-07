use std::{
    cmp,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = BufReader::new(File::open("input").unwrap())
        .lines()
        .map(|s| {
            s.unwrap()
                .split(" -> ")
                .map(|t| {
                    t.split(',')
                        .map(|u| u.parse().unwrap())
                        .collect::<Vec<i16>>()
                })
                .collect::<Vec<Vec<i16>>>()
        });

    let mut board = HashMap::new();

    for line in input {
        let (point1, point2) = (&line[0], &line[1]);
        if point1[0] == point2[0] {
            for point in cmp::min(point1[1], point2[1])..=cmp::max(point1[1], point2[1]) {
                *board.entry((point1[0], point)).or_insert(0) += 1;
            }
        } else if point1[1] == point2[1] {
            for point in cmp::min(point1[0], point2[0])..=cmp::max(point1[0], point2[0]) {
                *board.entry((point, point1[1])).or_insert(0) += 1;
            }
        }
    }

    println!(
        "{}",
        board.values().filter(|count| { **count >= 2 }).count()
    );
}
