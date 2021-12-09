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
        } else if (point1[0] - point2[0]).abs() == (point1[1] - point2[1]).abs() {
            if point1[0] <= point2[0] {
                let increment = if point1[1] <= point2[1] { 1 } else { -1 };
                let mut y_pos = point1[1];
                for x_pos in point1[0]..=point2[0] {
                    *board.entry((x_pos, y_pos)).or_insert(0) += 1;
                    y_pos += increment;
                }
            } else {
                let increment = if point2[1] <= point1[1] { 1 } else { -1 };
                let mut y_pos = point2[1];
                for x_pos in point2[0]..=point1[0] {
                    *board.entry((x_pos, y_pos)).or_insert(0) += 1;
                    y_pos += increment;
                }
            }
        }
    }

    println!(
        "Count: {}",
        board.values().filter(|count| { **count >= 2 }).count()
    );

    // for (coord, count) in board.into_iter().filter(|(coord, count)| { *count >= 2 }) {
    //     println!("({}, {})", coord.0, coord.1);
    // }
}
