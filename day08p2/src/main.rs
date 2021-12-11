use std::fs;

fn main() {
    let inputs = fs::read_to_string("input").unwrap();

    let inputs: Vec<(&str, &str)> = inputs
        .lines()
        .map(|entry| {
            let mut entry = entry.split(" | ");
            (entry.next().unwrap(), entry.next().unwrap())
        })
        .collect();

    let mut numbers = vec![];

    for (signals, outputs) in inputs {
        let mut decoded = [""; 10];
        let mut five_segments = Vec::with_capacity(3);
        let mut six_segments = Vec::with_capacity(3);

        for digit in signals.split(' ') {
            match digit.chars().count() {
                2 => decoded[1] = digit,
                3 => decoded[7] = digit,
                4 => decoded[4] = digit,
                5 => five_segments.push(digit),
                6 => six_segments.push(digit),
                7 => decoded[8] = digit,
                _ => {}
            }
        }

        // The digits with 5 segments are 2, 3, and 5
        // 3 and 5 share 3 segments with 4 - 2 only shares 2 segments with 4
        // 3 shares 2 segments with 1, but 5 only shares 1 segment with 1
        for digit in five_segments {
            if decoded[4].chars().filter(|c| digit.contains(*c)).count() == 3 {
                if decoded[1].chars().filter(|c| digit.contains(*c)).count() == 2 {
                    decoded[3] = digit;
                } else {
                    decoded[5] = digit;
                }
            } else {
                decoded[2] = digit;
            }
        }

        // The digits with 6 segments are 0, 6, and 9
        // 9 is a 5, with the remaining segment shared with 1
        // 6 is a 5, with the remaining segment not shared with 1
        // 0 has 1 unshared segment with 5
        for digit in six_segments {
            if digit.chars().filter(|c| !decoded[5].contains(*c)).count() == 1 {
                let oddity = digit.chars().find(|c| !decoded[5].contains(*c)).unwrap();
                if decoded[1].contains(oddity) {
                    decoded[9] = digit;
                } else {
                    decoded[6] = digit;
                }
            } else {
                decoded[0] = digit;
            }
        }

        let mut num = String::new();

        for digit in outputs.split(' ') {
            match digit.chars().count() {
                2 => num.push('1'),
                3 => num.push('7'),
                4 => num.push('4'),
                // There might have been a faster way to do this, like another mapping/HashMap to get the corresponding number
                5 => {
                    if digit.chars().filter(|c| decoded[2].contains(*c)).count() == 5 {
                        num.push('2')
                    } else if digit.chars().filter(|c| decoded[3].contains(*c)).count() == 5 {
                        num.push('3')
                    } else {
                        num.push('5')
                    }
                }
                6 => {
                    if digit.chars().filter(|c| decoded[0].contains(*c)).count() == 6 {
                        num.push('0')
                    } else if digit.chars().filter(|c| decoded[6].contains(*c)).count() == 6 {
                        num.push('6')
                    } else {
                        num.push('9')
                    }
                }
                7 => num.push('8'),
                _ => {}
            }
        }

        numbers.push(num.parse::<u64>().unwrap());
    }

    println!("SUM: {}", numbers.into_iter().sum::<u64>());
}
