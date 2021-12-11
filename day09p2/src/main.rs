use std::fs;

fn main() {
    let inputs = fs::read_to_string("input").unwrap();
    let board = inputs
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c as u32 - 48).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let low_points = find_low_points(&board);

    let mut basin_sizes = vec![];
    let mut marked_territory = vec![vec![false; board[0].len()]; board.len()];

    for (x, y) in low_points {
        basin_sizes.push(calculate_basin_size(
            (x as i32, y as i32),
            &board,
            &mut marked_territory,
        ));
    }

    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}

fn calculate_basin_size(
    coordinates: (i32, i32),
    board: &Vec<Vec<u32>>,
    marked_territory: &mut Vec<Vec<bool>>,
) -> u32 {
    let r = coordinates.0;
    let c = coordinates.1;
    if let Some(visited) = marked_territory.has(r, c) {
        if visited {
            0
        } else if let Some(num) = board.has(r, c) {
            if num == 9 {
                0
            } else {
                // This looks... not the cleanest and unidiomatic. The problem might be to many different cases
                // that just default to 0. Also redundant, but this might be just a result of the wa I set things
                // up in part 1.
                marked_territory[r as usize][c as usize] = true;
                let left = if board.has(r - 1, c).is_some() && num < board.has(r - 1, c).unwrap() {
                    calculate_basin_size((r - 1, c), board, marked_territory)
                } else {
                    0
                };
                let right = if board.has(r + 1, c).is_some() && num < board.has(r + 1, c).unwrap() {
                    calculate_basin_size((r + 1, c), board, marked_territory)
                } else {
                    0
                };
                let up = if board.has(r, c + 1).is_some() && num < board.has(r, c + 1).unwrap() {
                    calculate_basin_size((r, c + 1), board, marked_territory)
                } else {
                    0
                };
                let down = if board.has(r, c - 1).is_some() && num < board.has(r, c - 1).unwrap() {
                    calculate_basin_size((r, c - 1), board, marked_territory)
                } else {
                    0
                };
                1 + left + right + up + down
            }
        } else {
            0
        }
    } else {
        0
    }
}

// Part One Logic, moved for readability
fn find_low_points(board: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut low_points = vec![];

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
            low_points.push((r as usize, c as usize));
        }
    }

    low_points
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
