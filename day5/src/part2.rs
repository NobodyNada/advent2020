use std::io::{self, BufRead};

pub fn run() {
    const MAX: usize = 1024; // asume no ID is higher than 1024

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

    let mut exists = [false; MAX];
    ids.for_each(|id| exists[id] = true);

    let id = (1..MAX-1).find(|id| !exists[*id] && exists[*id-1] && exists[*id+1]);
    println!("{}", id.expect("no matching ID found"));
}
