use std::fs;

struct BreedingPeriod {
    internal_timer: u64,
    lantern_fish: u64,
}

impl BreedingPeriod {
    fn new(internal_timer: u64) -> Self {
        BreedingPeriod {
            internal_timer,
            lantern_fish: 0,
        }
    }
}

fn main() {
    let input: Vec<u64> = fs::read_to_string("test")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<u64>().unwrap())
        .collect();

    let mut lantern_fish_periods = vec![
        BreedingPeriod::new(1),
        BreedingPeriod::new(2),
        BreedingPeriod::new(3),
        BreedingPeriod::new(4),
        BreedingPeriod::new(5),
    ];

    for num in input {
        lantern_fish_periods[(num - 1) as usize].lantern_fish += 1;
    }

    for _days in 0..80 {
        let mut new_periods = vec![];
        for mut breeding_period in &mut lantern_fish_periods {
            if breeding_period.internal_timer == 0 {
                let mut t = BreedingPeriod::new(8);
                t.lantern_fish = breeding_period.lantern_fish;
                new_periods.push(t);
                breeding_period.internal_timer = 6;
            } else {
                breeding_period.internal_timer -= 1;
            }
        }
        lantern_fish_periods.append(&mut new_periods);
    }

    let mut lantern_fish = 0;
    for period in lantern_fish_periods {
        lantern_fish += period.lantern_fish;
    }

    println!("{}", lantern_fish);
}
