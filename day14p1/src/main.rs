use std::{
    fs::File,
    io::{self, BufRead}, collections::HashMap,
};

fn main() {
    let (mut template, rules) = parse_input("inputs/input");

    for _i in 0..10 {
        template = insert(template, &rules);
    }

    let mut char_count = HashMap::new();

    for c in template.chars() {
        if c.is_alphabetic() {
            *char_count.entry(c).or_insert(0) += 1;
        }
    }

    let max = char_count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    let min =  char_count.iter().max_by(|a, b| b.1.cmp(a.1)).unwrap().1;

    println!("{}", max - min);
}

fn parse_input(file: &str) -> (String, HashMap<Vec<char>, String>) {
    let mut lines = io::BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|r| r.unwrap());
    
    let template = lines.next().unwrap();
    lines.next();

    let mut rules = HashMap::new();
    for rule in lines {
        let rule = rule.split(" -> ").collect::<Vec<&str>>();
        let key = rule[0].chars().collect::<Vec<char>>();
        rules.insert(key, String::from(rule[1]));
    }

    (template, rules)
}

fn insert(template: String, rules: &HashMap<Vec<char>, String>) -> String {
    let mut string_builder = template.chars().take(1).collect::<String>();
    for pair in template.chars().collect::<Vec<char>>().windows(2) {
        if let Some(letter) = rules.get(pair) {
            string_builder.push_str(letter);
        }
        string_builder.push(pair[1]);
    }
    string_builder
}

#[test]
fn test_case_1() {
    let (mut template, rules) = parse_input("inputs/test");

    for _i in 0..10 {
        template = insert(template, &rules);
    }

    let mut char_count = HashMap::new();

    for c in template.chars() {
        if c.is_alphabetic() {
            *char_count.entry(c).or_insert(0) += 1;
        }
    }

    let max = char_count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    let min =  char_count.iter().max_by(|a, b| b.1.cmp(a.1)).unwrap().1;

    assert_eq!(max - min, 1588);
}