use std::fs;

fn main() {
    let inputs = fs::read_to_string("input").unwrap();
    let inputs = inputs
        .lines()
        .filter(|line| !line.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>());

    let mut scores = vec![];
    'to_next_line: for line in inputs {
        let mut check = vec![];
        for c in line {
            match c {
                '(' | '[' | '{' | '<' => {
                    check.push(c);
                }
                ')' => {
                    if let Some(d) = check.last() {
                        if *d != '(' {
                            continue 'to_next_line;
                        }
                        check.pop();
                    }
                }
                ']' => {
                    if let Some(d) = check.last() {
                        if *d != '[' {
                            continue 'to_next_line;
                        }
                        check.pop();
                    }
                }
                '}' => {
                    if let Some(d) = check.last() {
                        if *d != '{' {
                            continue 'to_next_line;
                        }
                        check.pop();
                    }
                }
                '>' => {
                    if let Some(d) = check.last() {
                        if *d != '<' {
                            continue 'to_next_line;
                        }
                        check.pop();
                    }
                }
                _ => {
                    println!("Unexpected character: {}", c)
                }
            }
        }
        check.reverse();

        let mut score: u64 = 0;
        for c in &check {
            score *= 5;
            match c {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => {
                    println!("Unexpected character: {}", c)
                }
            }
        }
        scores.push(score);
    }
    scores.sort_unstable();
    println!("{}", scores[scores.len() / 2]);
}
