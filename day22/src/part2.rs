use std::io::{self, BufRead};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct GameState {
    p1: VecDeque<i32>,
    p2: VecDeque<i32>
}

#[derive(Debug)]
enum Player {
    P1,
    P2
}

#[derive(Debug)]
enum RoundResult {
    Win(Player),
    Continue,
}

impl GameState {
    // false = p1, true = p2
    fn run_game(mut self) -> (Player, i32) {
        let mut history = HashSet::new();
        let winner = loop {
            match self.run_round(&mut history) {
                RoundResult::Continue => continue,
                RoundResult::Win(player) => break player
            }
        };

        let winning_deck = match winner {
            Player::P1 => self.p1,
            Player::P2 => self.p2
        };

        (
            winner,
            winning_deck.into_iter().rev()
                .enumerate().map(|(index, card)| (index as i32 + 1) * card)
                .sum()
        )
    }

    fn run_round(&mut self, history: &mut HashSet<GameState>) -> RoundResult {
        if history.contains(self)  { RoundResult::Win(Player::P1) }
        else if self.p1.is_empty() { RoundResult::Win(Player::P2) }
        else if self.p2.is_empty() { RoundResult::Win(Player::P1) }
        else {
            history.insert(self.clone());
            let (c1, c2) = (
                self.p1.pop_front().unwrap(),
                self.p2.pop_front().unwrap()
            );
            let winner = {
                if self.p1.len() as i32 >= c1 && self.p2.len() as i32 >= c2 {
                    // recurse
                    GameState {
                        p1: self.p1.iter().take(c1 as usize).cloned().collect(),
                        p2: self.p2.iter().take(c2 as usize).cloned().collect(),
                    }.run_game().0
                } else { match c1.cmp(&c2) {
                    std::cmp::Ordering::Greater => Player::P1,
                    std::cmp::Ordering::Less => Player::P2,
                    _ => panic!("tie")
                }}
            };

            match winner {
                Player::P1 => {
                    self.p1.push_back(c1);
                    self.p1.push_back(c2);
                },
                Player::P2 => {
                    self.p2.push_back(c2);
                    self.p2.push_back(c1);
                }
            };

            RoundResult::Continue
        }
    }
}

pub fn run() {
    let stdin = io::stdin();
    let input = stdin.lock().lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("read error");

    let mut input = input
        .split(|line| line.is_empty())
        .map(|player|
            player.iter().skip(1)
            .map(|line| line.parse::<i32>().expect("invalid input"))
            .collect::<VecDeque<i32>>()
    );

    let game = GameState {
        p1: input.next().unwrap(),
        p2: input.next().unwrap()
    };
    println!("{}", game.run_game().1);
}
