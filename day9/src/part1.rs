use std::io::{self, BufRead};

pub fn run() {
    const SIZE: usize = 25;
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line|
        line.expect("read error").parse::<usize>().expect("invalid input")
    ).collect::<Vec<_>>();
    for window in input.windows(SIZE + 1) {
        let target = window.last().unwrap();
        let preceding = &window[0..SIZE];

        let found = preceding.iter().find(|&i|
            preceding.iter().find(|&j|
                i + j == *target
            ) != None
        ) != None;

        if !found {
            println!("{}", target);
            break;
        }
    }
}
