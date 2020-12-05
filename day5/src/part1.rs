use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();
    let ids = stdin.lock().lines().map(|line|
        line.expect("read error").chars().fold(0, |n, c|
            (n << 1) | match c {
                'F'|'L' => 0,
                'B'|'R' => 1,
                _ => panic!("invalid char '{}'", c)
            }
        )
    );
    println!("{}", ids.max().expect("no boarding passes in input"));
}
