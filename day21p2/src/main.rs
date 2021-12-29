use std::cmp;

#[derive(Clone, Copy)]
struct Player {
    position: u32,
    score: u32,
}

impl Player {
    fn roll(&mut self, distance: u32) -> Self {
        let mut p = Player {
            position: self.position,
            score: self.score,
        };

        p.position = (p.position + distance) % 10;
        if p.position == 0 {
            p.position = 10
        }
        p.score += p.position;

        p
    }
}

fn main() {
    // Like day 17, not doing input reading
    let player_one = Player {
        position: 9,
        score: 0,
    };

    let player_two = Player {
        position: 3,
        score: 0,
    };

    let (player_one_wins, player_two_wins) = simulate_games(player_one, player_two);

    println!(
        "WINNER WIN COUNT: {}",
        cmp::max(player_one_wins, player_two_wins)
    );
}

fn simulate_games(mut player_one: Player, player_two: Player) -> (u64, u64) {
    // Roll distribution from rolling a 3-sided die 3 times
    //  Value      Frequency
    //------------------------
    //    3     |       1
    //    4     |       3
    //    5     |       6
    //    6     |       7
    //    7     |       6
    //    8     |       3
    //    9     |       1
    if player_one.score >= 21 {
        return (1, 0);
    } else if player_two.score >= 21 {
        return (0, 1);
    }

    let mut wins = (0, 0);

    let roll_three = simulate_games(player_two, player_one.roll(3));
    wins.0 += roll_three.1;
    wins.1 += roll_three.0;

    let roll_four = simulate_games(player_two, player_one.roll(4));
    wins.0 += roll_four.1 * 3;
    wins.1 += roll_four.0 * 3;

    let roll_five = simulate_games(player_two, player_one.roll(5));
    wins.0 += roll_five.1 * 6;
    wins.1 += roll_five.0 * 6;

    let roll_six = simulate_games(player_two, player_one.roll(6));
    wins.0 += roll_six.1 * 7;
    wins.1 += roll_six.0 * 7;

    let roll_seven = simulate_games(player_two, player_one.roll(7));
    wins.0 += roll_seven.1 * 6;
    wins.1 += roll_seven.0 * 6;

    let roll_eight = simulate_games(player_two, player_one.roll(8));
    wins.0 += roll_eight.1 * 3;
    wins.1 += roll_eight.0 * 3;

    let roll_nine = simulate_games(player_two, player_one.roll(9));
    wins.0 += roll_nine.1;
    wins.1 += roll_nine.0;

    wins
}

#[test]
fn example() {
    let player_one = Player {
        position: 4,
        score: 0,
    };

    let player_two = Player {
        position: 8,
        score: 0,
    };

    let (player_one_wins, player_two_wins) = simulate_games(player_one, player_two);

    assert_eq!(444356092776315, cmp::max(player_one_wins, player_two_wins));
    assert_eq!(341960390180808, cmp::min(player_one_wins, player_two_wins));
}
