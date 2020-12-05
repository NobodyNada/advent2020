use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();
    let ids = stdin.lock().lines().map(|line| {
        let mut row = 0;
        let mut col = 0;
        for c in line.expect("read error").chars() {
            match c {
                'F' => row = (row << 1) | 0,
                'B' => row = (row << 1) | 1,
                'L' => col = (col << 1) | 0,
                'R' => col = (col << 1) | 1,
                _ => panic!("invalid char '{}'", c)
            }
        }
        return row*8 + col;
    });
    println!("{}", ids.max().expect("no boarding passes in input"));
}
