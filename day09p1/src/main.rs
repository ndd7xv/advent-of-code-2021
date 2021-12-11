use std::fs;

fn main() {
    let inputs = fs::read_to_string("input").unwrap();
    let board = inputs
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c as u32 - 48).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let mut risk_levels_sum = 0;

    for r in 0..board.len() {
        for c in 0..board[0].len() {
            let r = r as i32;
            let c = c as i32;
            let low_point = board.has(r, c).unwrap();
            if let Some(num) = board.has(r - 1, c) {
                if low_point >= num {
                    continue;
                }
            }
            if let Some(num) = board.has(r + 1, c) {
                if low_point >= num {
                    continue;
                }
            }
            if let Some(num) = board.has(r, c - 1) {
                if low_point >= num {
                    continue;
                }
            }
            if let Some(num) = board.has(r, c + 1) {
                if low_point >= num {
                    continue;
                }
            }
            risk_levels_sum += low_point + 1;
        }
    }

    println!("{}", risk_levels_sum);
}

// Dipping my feet in traits, though my solution to this probably isn't the most efficient
// Parameters are i32s, which could potentially be generalized even more
// has(r, c) returns the Option<T> specified in at (r, c) in a 2D array. If it does not exist or its out of bounds, returns none.
trait SafeAccess<T> {
    fn has(&self, r: i32, c: i32) -> Option<T>;
}

impl<T> SafeAccess<T> for Vec<Vec<T>>
where
    T: Copy,
{
    fn has(&self, r: i32, c: i32) -> Option<T> {
        if r < 0
            || r as usize >= self.len()
            || c < 0
            || (!self.is_empty() && c as usize >= self[0].len())
        {
            return None;
        }
        if let Some(row) = self.get(r as usize) {
            if let Some(digit) = row.get(c as usize) {
                return Some(*digit);
            }
        }
        None
    }
}
