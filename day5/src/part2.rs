use std::io::{self, BufRead};

pub fn run() {
    const MAX: usize = 1024; // asume no ID is higher than 1024

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
    }).collect::<Vec<_>>();
    let mut exists = [false; MAX]; // assumes no ID is higher than 1024
    ids.iter().for_each(|id| exists[*id] = true);

    let id = (0..MAX).find(|id| *id != 0 && *id != MAX && 
        exists[*id-1] && !exists[*id] && exists[*id+1]);
    println!("{}", id.expect("no matching ID found"));
}
