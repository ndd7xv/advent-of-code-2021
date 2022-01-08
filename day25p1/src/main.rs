use std::{
    fs::File,
    io::{self, BufRead},
};

struct Map {
    board: Vec<Vec<Tile>>,
    step_count: i32,
}

#[derive(Copy, Clone, PartialEq)]
enum Cucumber {
    South,
    East,
}

type Tile = Option<Cucumber>;

impl Map {
    fn step(&mut self) -> bool {
        let mut changed = false;

        let mut new_board = vec![vec![None; self.board[0].len()]; self.board.len()];

        for (r, col) in self.board.iter().enumerate() {
            for (c, &cucumber) in col.iter().enumerate() {
                if let Some(Cucumber::East) = cucumber {
                    if self.board[r][(c + 1) % self.board[0].len()].is_none() {
                        new_board[r][(c + 1) % self.board[0].len()] = Some(Cucumber::East);
                        changed = true;
                    } else {
                        new_board[r][c] = cucumber;
                    }
                }
            }
        }

        for (r, col) in self.board.iter().enumerate() {
            for (c, &cucumber) in col.iter().enumerate() {
                if let Some(Cucumber::South) = cucumber {
                    if Some(Cucumber::South) == self.board[(r + 1) % self.board.len()][c]
                        || new_board[(r + 1) % self.board.len()][c].is_some()
                    {
                        new_board[r][c] = cucumber;
                    } else {
                        new_board[(r + 1) % self.board.len()][c] = cucumber;
                        changed = true;
                    }
                }
            }
        }

        self.board = new_board;
        self.step_count += 1;

        changed
    }
}

fn main() {
    let mut map = parse_input("inputs/input");
    while map.step() {}
    println!("COUNT: {}", map.step_count);
}

fn parse_input(path: &str) -> Map {
    let board = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| match c {
                    'v' => Some(Cucumber::South),
                    '>' => Some(Cucumber::East),
                    _ => None,
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    Map {
        board,
        step_count: 0,
    }
}
