use std::io::{self, BufRead};
use std::collections::VecDeque;

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

    let (mut p1, mut p2) = (
        input.next().unwrap(),
        input.next().unwrap()
    );
    assert!(input.next().is_none());

    while let (Some(&c1), Some(&c2)) = (p1.front(), p2.front()) {
        p1.pop_front();
        p2.pop_front();

        match c1.cmp(&c2) {
            std::cmp::Ordering::Greater => {
                p1.push_back(c1);
                p1.push_back(c2);
            },
            std::cmp::Ordering::Less => {
                p2.push_back(c2);
                p2.push_back(c1);
            },
            std::cmp::Ordering::Equal => {
                todo!("tie");
            }
        }
    }

    let winning_player;
    if p1.is_empty() { winning_player = p2; }
    else { winning_player = p1; }

    let score: i32 = winning_player.into_iter().rev()
        .enumerate().map(|(index, card)| (index as i32 + 1) * card)
        .sum();
    println!("{}", score);
}
