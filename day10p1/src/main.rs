use std::fs;

fn main() {
    let inputs = fs::read_to_string("input").unwrap();
    let inputs = inputs
        .lines()
        .filter(|line| !line.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>());

    let mut score = 0;
    for line in inputs {
        let mut check = vec![];
        for c in line {
            match c {
                '(' | '[' | '{' | '<' => {
                    check.push(c);
                }
                // Maybe I should try using unwrap_or_else as well, this logic may not work on empty stacks
                ')' => {
                    if let Some(d) = check.last() {
                        if *d == '(' {
                            check.pop();
                        } else {
                            score += 3;
                            break;
                        }
                    }
                }
                ']' => {
                    if let Some(d) = check.last() {
                        if *d == '[' {
                            check.pop();
                        } else {
                            score += 57;
                            break;
                        }
                    }
                }
                '}' => {
                    if let Some(d) = check.last() {
                        if *d == '{' {
                            check.pop();
                        } else {
                            score += 1197;
                            break;
                        }
                    }
                }
                '>' => {
                    if let Some(d) = check.last() {
                        if *d == '<' {
                            check.pop();
                        } else {
                            score += 25137;
                            break;
                        }
                    }
                }
                _ => {
                    println!("Unexpected character: {}", c)
                }
            }
        }
    }

    println!("SCORE: {}", score);
}
