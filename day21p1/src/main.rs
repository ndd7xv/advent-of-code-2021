use std::cmp;

struct Player {
    position: u32,
    score: u32,
}

// I could've sworn there was some sort of infinitely incrementing iterator,
// but this might be better in the long run depending on how the dice is simulated
struct Die {
    roll: u32,
    count: u32,
}

impl Die {
    fn new(start: u32) -> Self {
        Die {
            roll: start,
            count: 0,
        }
    }
    fn roll(&mut self) -> u32 {
        self.roll += 1;
        self.count += 1;
        self.roll - 1
    }
}

fn main() {
    // Like day 17, not doing input reading
    let mut player_one = Player {
        position: 9,
        score: 0,
    };

    let mut player_two = Player {
        position: 3,
        score: 0,
    };

    let mut die = Die::new(1);
    simulate_game(&mut player_one, &mut player_two, &mut die);

    println!(
        "LOSING SCORE * ROLL #: {}",
        cmp::min(player_one.score, player_two.score) * die.count
    );
}

fn simulate_game(player_one: &mut Player, player_two: &mut Player, die: &mut Die) {
    loop {
        let distance = die.roll() + die.roll() + die.roll();
        player_one.position = (player_one.position + distance) % 10;
        if player_one.position == 0 {
            player_one.position = 10
        }
        player_one.score += player_one.position;

        if player_one.score >= 1000 {
            break;
        }

        let distance = die.roll() + die.roll() + die.roll();
        player_two.position = (player_two.position + distance) % 10;
        if player_two.position == 0 {
            player_two.position = 10
        }
        player_two.score += player_two.position;

        if player_two.score >= 1000 {
            break;
        }
    }
}

#[test]
fn example() {
    let mut player_one = Player {
        position: 4,
        score: 0,
    };

    let mut player_two = Player {
        position: 8,
        score: 0,
    };

    let mut die = Die::new(1);
    simulate_game(&mut player_one, &mut player_two, &mut die);
    assert_eq!(
        739785,
        cmp::min(player_one.score, player_two.score) * die.count
    );
}
