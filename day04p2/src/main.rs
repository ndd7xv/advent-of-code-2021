use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Board {
    layout: Vec<Vec<isize>>,
    marked: Vec<(usize, usize)>,
    has_won: bool,
}

impl Board {
    fn new(layout: Vec<Vec<isize>>) -> Self {
        Board {
            layout,
            marked: vec![],
            has_won: false,
        }
    }
    fn exists(&mut self, number: isize) -> Option<isize> {
        // println!("Does {} exist on {:?}?", number, self.layout);
        let mut location = None;

        for (r, row) in self.layout.clone().into_iter().enumerate() {
            for (c, board_num) in row.into_iter().enumerate() {
                if board_num == number {
                    location = Some((r, c));
                }
            }
        }

        if let Some(location) = location {
            self.marked.push(location);
            // println!("Yes, on {} {}", location.0, location.1);
            let win = self.is_winner(number, location);
            if let Some(win) = win {
                return Some(win);
            }
        }
        None
    }
    fn is_winner(&mut self, number: isize, location: (usize, usize)) -> Option<isize> {
        let mut win = false;

        for r in 0..5 {
            // println!("({}, {}) in {:?}", r, location.1, self.marked);
            if !self.marked.contains(&(r, location.1)) {
                // println!("NO");
                break;
            } else if r == 4 {
                win = true;
            }
        }

        if !win {
            for c in 0..5 {
                if !self.marked.contains(&(location.0, c)) {
                    break;
                } else if c == 4 {
                    win = true;
                }
            }
        }

        if location.0 == location.1 && !win {
            for i in 0..5 {
                if !self.marked.contains(&(i, i)) {
                    break;
                } else if i == 4 {
                    win = true;
                }
            }
        }

        if win {
            // println!("check_winner: {:?} has won!", self.layout);
            self.has_won = true;
            Some(self.unmarked_sum() * number)
        } else {
            None
        }
    }
    fn unmarked_sum(&self) -> isize {
        let mut sum = 0;
        for r in 0..5 {
            for c in 0..5 {
                if !self.marked.contains(&(r, c)) {
                    sum += self.layout[r][c];
                }
            }
        }
        sum
    }
}

fn main() {
    let mut input = BufReader::new(File::open("input").unwrap())
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<String>>()
        .into_iter();

    let bingo_balls = input.next().unwrap();
    let bingo_balls = bingo_balls
        .split(',')
        .map(|f| f.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    let rows = input
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect()
        });

    let mut boards: Vec<Board> = vec![];
    let mut board: Vec<Vec<isize>> = vec![vec![]; 5];

    for (index, row) in rows.enumerate() {
        board[index % 5] = row;
        if index % 5 == 4 {
            boards.push(Board::new(board));
            board = vec![vec![]; 5];
        }
    }

    for ball in bingo_balls {
        // Probably better to leave this as an iterator that operates on the boards that haven't won yet
        let on_last_board = boards.iter().filter(|b| !b.has_won).count() == 1;
        for board in boards.iter_mut() {
            if !board.has_won {
                board.exists(ball);
                if board.has_won && on_last_board {
                    println!("Score of last board: {}", board.exists(ball).unwrap());
                }
            }
        }
    }
}
