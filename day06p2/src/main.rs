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
    let input: Vec<u64> = fs::read_to_string("input")
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

    for _days in 0..256 {
        let mut fish_to_breed = 0;

        // Simulate a day, keeping track of new and old fish to start a new breeding period
        for mut breeding_period in &mut lantern_fish_periods {
            if breeding_period.internal_timer == 0 {
                fish_to_breed += breeding_period.lantern_fish;
            }
            breeding_period.internal_timer = breeding_period.internal_timer.wrapping_sub(1);
        }
        
        // Remove all indexes of old breeding periods (internal_timer is u64::MAX)
        lantern_fish_periods = lantern_fish_periods.into_iter().filter(| breeding_period| breeding_period.internal_timer != u64::MAX).collect::<Vec<BreedingPeriod>>();

        // Search for an existing period to add fish to
        let mut sufficient_period = false;
        for mut breeding_period in &mut lantern_fish_periods {
            if breeding_period.internal_timer == 6 {
                breeding_period.lantern_fish += fish_to_breed;
                sufficient_period = true;
                break;
            }
        }
        if !sufficient_period {
            let mut t = BreedingPeriod::new(6);
            t.lantern_fish = fish_to_breed;
            lantern_fish_periods.push(t);
        }

        // Create a new period for the new fish
        let mut t = BreedingPeriod::new(8);
        t.lantern_fish =fish_to_breed;
        lantern_fish_periods.push(t);
    }

    let mut lantern_fish = 0;
    for period in lantern_fish_periods {
        lantern_fish += period.lantern_fish;
    }

    println!("{}", lantern_fish);
}
