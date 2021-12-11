use std::fs;

struct DumboOctopus {
    energy_level: u8,
    has_triggered: bool,
}

impl DumboOctopus {
    fn increment(&mut self) -> bool {
        if !self.has_triggered {
            self.energy_level += 1;
            if self.energy_level > 9 {
                self.has_triggered = true;
                self.energy_level = 0;
                return true;
            }
            return false;
        }
        false
    }
}

fn main() {
    let mut board = fs::read_to_string("input")
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|row| {
            row.chars()
                .map(|c| DumboOctopus {
                    energy_level: c as u8 - 48,
                    has_triggered: false,
                })
                .collect::<Vec<DumboOctopus>>()
        })
        .collect::<Vec<Vec<DumboOctopus>>>();

    let mut flashes = 0;
    for _i in 0..100 {
        flashes += step(&mut board);
    }
    println!("FLASHES: {}", flashes);
}

fn step(board: &mut Vec<Vec<DumboOctopus>>) -> u32 {
    let mut flashes = 0;

    for r in 0..board.len() {
        for c in 0..board[0].len() {
            flashes += increment(board, r as i8, c as i8);
        }
    }

    for row in board {
        for oct in row {
            oct.has_triggered = false;
        }
    }
    flashes
}

fn increment(board: &mut Vec<Vec<DumboOctopus>>, r: i8, c: i8) -> u32 {
    if r < 0 || r == board.len() as i8 || c < 0 || c == board[0].len() as i8 {
        return 0;
    }
    if board[r as usize][c as usize].increment() {
        return 1
            + increment(board, r - 1, c - 1)
            + increment(board, r - 1, c)
            + increment(board, r - 1, c + 1)
            + increment(board, r, c - 1)
            + increment(board, r, c + 1)
            + increment(board, r + 1, c - 1)
            + increment(board, r + 1, c)
            + increment(board, r + 1, c + 1);
    }
    0
}
