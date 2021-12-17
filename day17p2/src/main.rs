use std::{cmp, ops::Range};

fn main() {
    println!(
        "TOTAL VELOCITIES: {}",
        solve(155..(182 + 1), -117..(-67 + 1))
    );
}

fn find_x_increments(x_range: &Range<i32>) -> Vec<i32> {
    let mut x_coords = vec![];
    for potential_answer in 0..=x_range.clone().max().unwrap() {
        let mut pos = 0;
        for step in 0..=potential_answer {
            pos += potential_answer - step;
            if x_range.contains(&pos) && !x_coords.contains(&potential_answer) {
                x_coords.push(potential_answer)
            }
        }
    }
    x_coords
}

// I seem to clone to find a single number, maybe it would've been better to past endpoints as opposed to a range
fn find_y_count_given_x(x: i32, x_range: &Range<i32>, y_range: &Range<i32>) -> i32 {
    let (lower, upper) = (
        y_range.clone().min().unwrap(),
        y_range.clone().max().unwrap(),
    );
    let mut y_vel = cmp::max(lower.abs(), upper.abs());
    let mut velocities = vec![];
    while y_vel >= lower {
        let mut x_vel = x;
        let mut x_pos = 0;
        let mut y_pos = 0;
        let mut y_guess_vel = y_vel;

        while y_pos >= lower && x_pos <= x_range.clone().max().unwrap() {
            x_pos += x_vel;
            y_pos += y_guess_vel;

            if x_vel > 0 {
                x_vel -= 1
            }
            y_guess_vel -= 1;

            if y_range.contains(&y_pos) && x_range.contains(&x_pos) {
                if !velocities.contains(&(x, y_vel)) {
                    velocities.push((x, y_vel));
                }
                break;
            }
        }
        y_vel -= 1;
    }
    velocities.len() as i32
}

fn solve(x_range: Range<i32>, y_range: Range<i32>) -> i32 {
    let x_coords = find_x_increments(&x_range);
    let mut possibilities = 0;
    for x in x_coords {
        possibilities += find_y_count_given_x(x, &x_range, &y_range);
    }
    possibilities
}

#[test]
fn test_case_1() {
    assert_eq!(solve(20..(30 + 1), -10..(-5 + 1)), 112);
}
