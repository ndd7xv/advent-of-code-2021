use std::{
    cmp,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let board = parse_input("input/input");
    println!("Shortest Path: {}", solve(board));
}

fn parse_input(file: &str) -> Vec<Vec<i32>> {
    io::BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|r| {
            r.unwrap()
                .chars()
                .map(|f| f.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn solve(board: Vec<Vec<i32>>) -> i32 {
    let mut solutions = vec![vec![-1; board[0].len()]; board.len()];
    solutions[0][0] = 0;
    find_shortest_path(&board, &mut solutions, board.len() - 1, board[0].len() - 1)
}

// This doesn't even work correctly, I passed part 1 because my input doesn't trigger the situation I cannot handle
// I'm leaving this here for posterity - revel in my naivete! You can revel in my lack of idiomaticness in day15p2
fn find_shortest_path(
    board: &[Vec<i32>],
    solutions: &mut Vec<Vec<i32>>,
    r: usize,
    c: usize,
) -> i32 {
    if solutions[r][c] == -1 {
        let mut shortest = i32::MAX;
        if r != 0 {
            shortest = cmp::min(shortest, find_shortest_path(board, solutions, r - 1, c));
        }
        if c != 0 {
            shortest = cmp::min(shortest, find_shortest_path(board, solutions, r, c - 1));
        }
        solutions[r][c] = shortest + board[r][c];
        return solutions[r][c];
    }
    solutions[r][c]
}

#[test]
fn test_case_1() {
    let board = parse_input("input/test");
    assert_eq!(solve(board), 40);
}
