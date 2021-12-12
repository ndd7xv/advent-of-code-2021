use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let connections = parse_input("inputs/input");
    let answer = find_paths(&connections);
    println!("PATHS: {}", answer);
}

// I'm trying to restructure in the efforts of testing and readability; however, I probably need to figure a better way than just cloning Strings.
fn parse_input(file_path: &str) -> HashMap<String, Vec<String>> {
    let mut connections = HashMap::new();
    let lines = io::BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|r| r.unwrap())
        .filter(|line| !line.is_empty());

    for line in lines {
        let endpoints = line.split('-').map(String::from).collect::<Vec<String>>();
        connections
            .entry(endpoints[0].clone())
            .or_insert_with(Vec::new)
            .push(endpoints[1].clone());
        connections
            .entry(endpoints[1].clone())
            .or_insert_with(Vec::new)
            .push(endpoints[0].clone());
    }

    connections
}

// Behemoth of inefficiency, I should probably check how other people did this later
// Should I have just checked if I've already visited a small cave inside the method instead of passing a variable?
fn find_paths(connections: &HashMap<String, Vec<String>>) -> u32 {
    find_paths_helper(connections, &mut vec![String::from("start")], false)
}

fn find_paths_helper(
    connections: &HashMap<String, Vec<String>>,
    curr_path: &mut Vec<String>,
    has_visited_small: bool
) -> u32 {
    let mut count = 0;
    if let Some(neighbors) = connections.get(&String::from(curr_path.last().unwrap())) {
        for neighbor in neighbors {
            if neighbor.eq("end") {
                count += 1;
            } else if neighbor.to_lowercase().eq(neighbor) {
                if has_visited_small {
                    if !curr_path.contains(neighbor) {
                        curr_path.push(neighbor.to_string());
                        count += find_paths_helper(connections, curr_path, true);
                        curr_path.pop();
                    }
                } else {
                    if curr_path.iter().filter(| &f | f.eq(neighbor)).count() < 2 && !neighbor.eq("start") {
                        if curr_path.contains(neighbor) {
                            curr_path.push(neighbor.to_string());
                            count += find_paths_helper(connections, curr_path, true);
                        } else {
                            curr_path.push(neighbor.to_string());
                            count += find_paths_helper(connections, curr_path, false);
                        }
                        curr_path.pop();
                    }
                }
            } else {
                curr_path.push(neighbor.to_string());
                count += find_paths_helper(connections, curr_path, has_visited_small);
                curr_path.pop();
            }
        }
    }
    count
}

#[test]
fn test_case_1() {
    let connections = parse_input("inputs/test1");
    let answer = find_paths(&connections);
    assert_eq!(36, answer);
}

#[test]
fn test_case_2() {
    let connections = parse_input("inputs/test2");
    let answer = find_paths(&connections);
    assert_eq!(103, answer);
}

#[test]
fn test_case_3() {
    let connections = parse_input("inputs/test3");
    let answer = find_paths(&connections);
    assert_eq!(3509, answer);
}
