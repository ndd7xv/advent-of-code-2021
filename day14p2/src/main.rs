use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let (template, rules) = parse_input("inputs/input");

    let mut pair_count = HashMap::new();

    for pair in template.chars().collect::<Vec<char>>().windows(2) {
        *pair_count.entry([pair[0], pair[1]]).or_insert(0) += 1;
    }

    for _i in 0..40 {
        pair_count = step(&rules, &mut pair_count);
    }

    println!("RANGE: {}", calculate_range(template, &mut pair_count));
}

fn parse_input(file: &str) -> (String, HashMap<[char; 2], [[char; 2]; 2]>) {
    let mut lines = io::BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|r| r.unwrap());

    let template = lines.next().unwrap();
    lines.next();

    let mut rules = HashMap::new();
    for rule in lines {
        let rule = rule.split(" -> ").collect::<Vec<&str>>();
        let key = [
            rule[0].chars().next().unwrap(),
            rule[0].chars().last().unwrap(),
        ];

        let first_pair = [key[0], rule[1].chars().last().unwrap()];
        let second_pair = [rule[1].chars().next().unwrap(), key[1]];

        rules.insert(key, [first_pair, second_pair]);
    }

    (template, rules)
}

fn step(
    rules: &HashMap<[char; 2], [[char; 2]; 2]>,
    pair_count: &mut HashMap<[char; 2], u64>,
) -> HashMap<[char; 2], u64> {
    let mut updated_count = HashMap::new();

    for (&key, &mut value) in pair_count {
        if let Some(new_pairs) = rules.get(&key) {
            for pair in new_pairs {
                *updated_count.entry(*pair).or_insert(0) += value;
            }
        }
    }

    updated_count
}

fn calculate_range(template: String, pair_count: &mut HashMap<[char; 2], u64>) -> u64 {
    let mut char_count = HashMap::new();

    for (&pair, &mut count) in pair_count {
        for c in pair {
            *char_count.entry(c).or_insert(0) += count;
        }
    }

    char_count.insert(
        template.chars().nth(1).unwrap(),
        char_count
            .get(&template.chars().nth(1).unwrap())
            .unwrap()
            + 1,
    );
    char_count.insert(
        template.chars().last().unwrap(),
        char_count.get(&template.chars().last().unwrap()).unwrap() + 1,
    );

    let max = char_count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    let min = char_count.iter().max_by(|a, b| b.1.cmp(a.1)).unwrap().1;

    (max - min) / 2
}

#[test]
fn test_case_1() {
    let (template, rules) = parse_input("inputs/test");

    let mut pair_count = HashMap::new();

    for pair in template.chars().collect::<Vec<char>>().windows(2) {
        *pair_count.entry([pair[0], pair[1]]).or_insert(0) += 1;
    }

    for _i in 0..40 {
        pair_count = step(&rules, &mut pair_count);
    }

    assert_eq!(calculate_range(template, &mut pair_count), 2188189693529);
}

#[test]
fn old_test_case_1() {
    let (template, rules) = parse_input("inputs/test");

    let mut pair_count = HashMap::new();

    for pair in template.chars().collect::<Vec<char>>().windows(2) {
        *pair_count.entry([pair[0], pair[1]]).or_insert(0) += 1;
    }

    for _i in 0..10 {
        pair_count = step(&rules, &mut pair_count);
    }

    assert_eq!(calculate_range(template, &mut pair_count), 1588);
}

#[test]
fn basic() {
    let (template, rules) = parse_input("inputs/test");

    let mut pair_count = HashMap::new();

    for pair in template.chars().collect::<Vec<char>>().windows(2) {
        *pair_count.entry([pair[0], pair[1]]).or_insert(0) += 1;
    }

    for _i in 0..1 {
        pair_count = step(&rules, &mut pair_count);
    }

    assert_eq!(calculate_range(template, &mut pair_count), 1);
}
