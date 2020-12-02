use std::io::{self, BufRead};

pub fn run() {
    for line in io::stdin().lock().lines() {
        println!("{}", line.expect("read error"));
    }
    todo!()
}
