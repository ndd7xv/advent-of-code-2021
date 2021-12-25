use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead},
    ops::Add,
};

// Implement Pair Reduction
// Calculate Magnitude
#[derive(Clone)]
enum Data {
    Pair(Box<[Pair; 2]>),
    Value(u32),
}

impl Data {
    fn format(&self) -> String {
        let mut buffer = String::new();

        match self {
            Data::Pair(pair) => {
                buffer.push('[');
                buffer.push_str(&pair[0].format()[..]);
                buffer.push(',');
                buffer.push_str(&pair[1].format()[..]);
                buffer.push(']');
            }
            Data::Value(i) => {
                buffer.push_str(&i.to_string()[..]);
            }
        }

        buffer
    }
    fn increment(&mut self) {
        if let Data::Pair(p) = self {
            p[0].increment();
            p[1].increment();
        }
    }
}

#[derive(Clone)]
struct Pair {
    data: Data,
    level: u32,
}

impl Pair {
    fn from(str: &str) -> Self {
        let mut data_creator: Vec<Pair> = vec![];
        str.chars().for_each(|c| match c {
            ']' => {
                if data_creator.len() > 1 {
                    let (mut second, mut first) =
                        (data_creator.pop().unwrap(), data_creator.pop().unwrap());
                    first.increment();
                    second.increment();
                    let pair = Box::new([first, second]);
                    data_creator.push(Pair {
                        data: Data::Pair(pair),
                        level: 1,
                    });
                }
            }
            '0'..='9' => data_creator.push(Pair {
                data: Data::Value(c.to_digit(10).unwrap()),
                level: 1,
            }),
            _ => {}
        });

        data_creator.pop().unwrap()
    }
    fn increment(&mut self) {
        self.level += 1;
        self.data.increment();
    }
    fn format(&self) -> String {
        // let mut buffer = String::from(format!("({})", self.level));
        let mut buffer = String::new();
        buffer.push_str(&self.data.format()[..]);
        //buffer.push_str(&format!("({})", self.level)[..]);
        buffer
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Add for Pair {
    type Output = Pair;
    fn add(mut self, mut rhs: Self) -> Self {
        self.increment();
        rhs.increment();
        let data = Data::Pair(Box::new([self, rhs]));
        let p = Pair { data, level: 1 };

        reduce(p)
    }
}

fn main() {
    let pairs = parse_input("inputs/input");
    let sum = sum(pairs);
    println!("MAGNITUDE: {}", get_magnitude(&sum));
}

fn parse_input(file: &str) -> Vec<Pair> {
    io::BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|r| {
            let string = r.unwrap();
            Pair::from(&string[..])
        })
        .collect::<Vec<Pair>>()
}

fn sum(pairs: Vec<Pair>) -> Pair {
    let mut pairs = pairs.into_iter();
    let mut sum = pairs.next().unwrap();
    for pair in pairs {
        sum = sum + pair;
    }
    sum
}

fn reduce(pair: Pair) -> Pair {
    let (mut p, _l, _r, mut b) = check_for_nest_limit(&pair);

    while !b {
        let (tp, _l, _r, tb) = check_for_nest_limit(&p);
        p = tp;
        b = tb;
    }

    let (tp, tb) = check_for_value_limit(&p);
    p = tp;
    b = tb;

    if !b {
        return reduce(p);
    }

    p
}

fn check_for_nest_limit(pair: &Pair) -> (Pair, i32, i32, bool) {
    let mut l = -1;
    let mut r = -1;
    if let Data::Pair(mut p) = pair.data.clone() {
        if pair.level < 5 {
            let (mut a, a_left, a_right, ab) = check_for_nest_limit(&p[0]);

            if a_right < 0 && a_left < 0 && !ab {
                let data = Data::Pair(Box::new([a, p[1].clone()]));
                let tp = Pair {
                    data,
                    level: pair.level,
                };
                return (tp, l, r, ab);
            }

            if a_right >= 0 {
                p[1] = add_left(&mut p[1], a_right as u32);
                if a_left < 0 {
                    let data = Data::Pair(Box::new([a, p[1].clone()]));
                    let tp = Pair {
                        data,
                        level: pair.level,
                    };
                    return (tp, l, r, ab);
                }
            }

            if a_left >= 0 {
                l = a_left;
                let data = Data::Pair(Box::new([a, p[1].clone()]));
                let tp = Pair {
                    data,
                    level: pair.level,
                };
                return (tp, l, r, ab);
            }

            let (b, b_left, b_right, bb) = check_for_nest_limit(&p[1]);

            if b_left < 0 && b_right < 0 && !bb {
                let data = Data::Pair(Box::new([a, b]));
                let tp = Pair {
                    data,
                    level: pair.level,
                };
                return (tp, l, r, bb);
            }

            if b_left >= 0 {
                a = add_right(&mut p[0], b_left as u32);
                if b_right < 0 {
                    let data = Data::Pair(Box::new([a, b]));
                    let tp = Pair {
                        data,
                        level: pair.level,
                    };
                    return (tp, l, r, bb);
                }
            }

            if b_right >= 0 {
                r = b_right;
                let data = Data::Pair(Box::new([a, b]));
                let tp = Pair {
                    data,
                    level: pair.level,
                };
                return (tp, l, r, bb);
            }

            let data = Data::Pair(Box::new([a, b]));
            let tp = Pair {
                data,
                level: pair.level,
            };
            (tp, l, r, bb && ab)
        } else {
            let tp = Pair {
                data: Data::Value(0),
                level: pair.level,
            };
            if let Data::Value(i) = p[0].data {
                l = i as i32;
            }
            if let Data::Value(i) = p[1].data {
                r = i as i32;
            }
            (tp, l, r, false)
        }
    } else if let Data::Value(v) = pair.data {
        let p = Pair {
            data: Data::Value(v),
            level: pair.level,
        };
        (p, l, r, true)
    } else {
        let p = Pair {
            data: Data::Value(0),
            level: pair.level,
        };
        (p, l, r, true)
    }
}

fn check_for_value_limit(pair: &Pair) -> (Pair, bool) {
    if let Data::Value(v) = pair.data {
        if v >= 10 {
            let a = Pair {
                data: Data::Value(v / 2),
                level: pair.level + 1,
            };
            let b = Pair {
                data: Data::Value(((v as f64 + 1.0) / 2.0) as u32),
                level: pair.level + 1,
            };
            let p = Pair {
                data: Data::Pair(Box::new([a, b])),
                level: pair.level,
            };
            (p, false)
        } else {
            let p = Pair {
                data: Data::Value(v),
                level: pair.level,
            };
            (p, true)
        }
    } else if let Data::Pair(p) = &pair.data {
        let (left, left_unchanged) = check_for_value_limit(&p[0]);

        if !left_unchanged {
            let data = Data::Pair(Box::new([left, p[1].clone()]));
            let tp = Pair {
                data,
                level: pair.level,
            };
            return (tp, left_unchanged);
        }

        let (right, right_unchanged) = check_for_value_limit(&p[1]);

        let data = Data::Pair(Box::new([left, right]));
        let tp = Pair {
            data,
            level: pair.level,
        };
        (tp, right_unchanged)
    } else {
        let p = Pair {
            data: Data::Value(0),
            level: pair.level,
        };
        (p, false)
    }
}

fn add_left(pair: &mut Pair, value: u32) -> Pair {
    if let Data::Value(i) = pair.data {
        Pair {
            data: Data::Value(i + value),
            level: pair.level,
        }
    } else if let Data::Pair(mut p) = pair.data.clone() {
        let l = add_left(&mut p[0], value);
        let r = p[1].clone();
        let data = Data::Pair(Box::new([l, r]));
        Pair {
            data,
            level: pair.level,
        }
    } else {
        Pair {
            data: Data::Value(value),
            level: 1,
        }
    }
}

fn add_right(pair: &mut Pair, value: u32) -> Pair {
    if let Data::Value(i) = pair.data {
        Pair {
            data: Data::Value(i + value),
            level: pair.level,
        }
    } else if let Data::Pair(mut p) = pair.data.clone() {
        let r = add_right(&mut p[1], value);
        let l = p[0].clone();
        let data = Data::Pair(Box::new([l, r]));
        Pair {
            data,
            level: pair.level,
        }
    } else {
        Pair {
            data: Data::Value(value),
            level: 1,
        }
    }
}
fn get_magnitude(pair: &Pair) -> u32 {
    match &pair.data {
        Data::Pair(pairs) => 3 * get_magnitude(&pairs[0]) + 2 * get_magnitude(&pairs[1]),
        Data::Value(i) => *i,
    }
}

#[test]
fn simple_add() {
    let pairs = parse_input("inputs/simple_add");
    let sum = sum(pairs);
    assert_eq!(
        format!("{}", sum),
        String::from("[[[[3,0],[5,3]],[4,4]],[5,5]]")
    );
}

#[test]
fn complex_plus_simple() {
    let pairs = parse_input("inputs/complex_simple");
    let sum = sum(pairs);
    assert_eq!(
        format!("{}", sum),
        String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );
}

#[test]
fn complex_add() {
    let pair = parse_input("inputs/complex_add");
    assert_eq!(
        format!("{}", sum(pair)),
        String::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
    );
}

#[test]
fn magnitude_1() {
    let pair = Pair::from("[[1,2],[[3,4],5]]");
    assert_eq!(get_magnitude(&pair), 143);
}

#[test]
fn magnitude_2() {
    let pair = Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    assert_eq!(get_magnitude(&pair), 1384);
}

#[test]
fn magnitude_3() {
    let pair = Pair::from("[[[[1,1],[2,2]],[3,3]],[4,4]]");
    assert_eq!(get_magnitude(&pair), 445);
}

#[test]
fn magnitude_4() {
    let pair = Pair::from("[[[[3,0],[5,3]],[4,4]],[5,5]]");
    assert_eq!(get_magnitude(&pair), 791);
}

#[test]
fn magnitude_5() {
    let pair = Pair::from("[[[[5,0],[7,4]],[5,5]],[6,6]]");
    assert_eq!(get_magnitude(&pair), 1137);
}

#[test]
fn magnitude_6() {
    let pair = Pair::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    assert_eq!(get_magnitude(&pair), 3488);
}


