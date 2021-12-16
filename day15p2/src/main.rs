use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::Debug,
    fs::File,
    io::{self, BufRead},
};

#[derive(Eq, PartialEq)]
struct Node {
    distance: i32,
    r: usize,
    c: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("distance", &self.distance)
            .field("r", &self.r)
            .field("c", &self.c)
            .finish()
    }
}

fn main() {
    let board = parse_input("input/input");
    let board = magnify(board, 5);
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

fn magnify(board: Vec<Vec<i32>>, magnitude: u32) -> Vec<Vec<i32>> {
    let mut new_board = board.clone();
    let mut incremental_board = board;

    for _i in 1..magnitude {
        incremental_board = incremental_board
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|a| {
                        let a = 1 + a;
                        if a > 9 {
                            1
                        } else {
                            a
                        }
                    })
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        for (index, row) in incremental_board.iter().enumerate() {
            new_board[index].append(&mut row.clone());
        }
    }

    let mut incremental_board = new_board.clone();
    for _i in 1..magnitude {
        incremental_board = incremental_board
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|a| {
                        let a = 1 + a;
                        if a > 9 {
                            1
                        } else {
                            a
                        }
                    })
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        for row in incremental_board.iter() {
            new_board.push(row.clone());
        }
    }
    new_board
}

fn solve(board: Vec<Vec<i32>>) -> i32 {
    let mut nodes = vec![vec![i32::MAX; board[0].len()]; board.len()];
    nodes[0][0] = 0;
    find_shortest_path(&board, &mut nodes)
}

// And they said I'd never use Djikstra's again!!!
fn find_shortest_path(board: &[Vec<i32>], solutions: &mut Vec<Vec<i32>>) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        distance: 0,
        r: 0,
        c: 0,
    });

    while let Some(Node { distance, r, c }) = heap.pop() {
        if r == board.len() - 1 && c == board[0].len() - 1 {
            return distance;
        }

        if distance > solutions[r][c] {
            continue;
        }

        let up = if r > 0 { Some(r - 1) } else { None };
        let down = if r < board.len() - 1 {
            Some(r + 1)
        } else {
            None
        };
        let left = if c > 0 { Some(c - 1) } else { None };
        let right = if c < board[0].len() - 1 {
            Some(c + 1)
        } else {
            None
        };

        if let Some(up) = up {
            let temp = Node {
                distance: distance + board[up][c],
                r: up,
                c,
            };
            if temp.distance < solutions[up][c] {
                solutions[up][c] = temp.distance;
                heap.push(temp);
            }
        }

        if let Some(down) = down {
            let temp = Node {
                distance: distance + board[down][c],
                r: down,
                c,
            };
            if temp.distance < solutions[down][c] {
                solutions[down][c] = temp.distance;
                heap.push(temp);
            }
        }

        if let Some(left) = left {
            let temp = Node {
                distance: distance + board[r][left],
                r,
                c: left,
            };
            if temp.distance < solutions[r][left] {
                solutions[r][left] = temp.distance;
                heap.push(temp);
            }
        }

        if let Some(right) = right {
            let temp = Node {
                distance: distance + board[r][right],
                r,
                c: right,
            };
            if temp.distance < solutions[r][right] {
                solutions[r][right] = temp.distance;
                heap.push(temp);
            }
        }
    }
    -1
}

#[test]
fn test_case_1() {
    let board = parse_input("input/test");
    let board = magnify(board, 5);
    assert_eq!(solve(board), 315);
}
#[test]
fn test_magnify() {
    let board = parse_input("input/test");
    let board = magnify(board, 5);
    assert_eq!(board, parse_input("input/test_expand"));
}

#[test]
fn basic() {
    let board = parse_input("input/basic");
    //let board = magnify(board, 5);
    assert_eq!(solve(board), 4);
}
