use std::{cmp, ops::Range};
// I'm opting to manually put in values since I don't think there's too much value
// in parsing the values that they've formatted
fn main() {
    println!("HIGHEST Y: {}", solve(155..182, -117..(-67 + 1)));
}

fn find_x_increments(x_range: Range<i32>) -> Range<i32> {
    let (start, end) = (x_range.clone().min().unwrap(), x_range.max().unwrap());
    let mut i = 0;
    while ((i + 1) * i) / 2 < start {
        i += 1;
    }
    let min = i;
    while ((i + 1) * i) / 2 < end {
        i += 1;
    }
    min..i
}

// I seem to clone to find a single number, maybe it would've been better to past endpoints as opposed to a range
fn find_max_y_given_x(x: i32, y_range: &Range<i32>) -> i32 {
    let (start, end) = (y_range.clone().min().unwrap(), y_range.clone().max().unwrap());
    let mut guessed_vel = cmp::max(start.abs(), end.abs());
    let mut max_y = i32::MIN;
    loop {
        let mut pos = 0;
        let mut vel = guessed_vel;
        for _step in 0..x {
            pos += vel;
            if vel == 0 {
                max_y = pos
            }
            vel -= 1;
        }

        while pos >= start || y_range.contains(&pos) {
            pos += vel;
            if vel == 0 {
                max_y = pos
            }
            vel -= 1;
        }

        if y_range.contains(&(pos - (vel + 1))) {
            return max_y;
        };
        guessed_vel -= 1;
    }
}

fn solve(x_range: Range<i32>, y_range: Range<i32>) -> i32 {
    let x_range_sol = find_x_increments(x_range);
    let mut sol = i32::MIN;
    for x in x_range_sol {
        sol = cmp::max(find_max_y_given_x(x, &y_range), sol);
    }
    sol
}

#[test]
fn test_case_1() {
    assert_eq!(solve(20..30, -10..(-5 + 1)), 45);
}
