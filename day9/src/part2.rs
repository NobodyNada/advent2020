use std::io::{self, BufRead};

pub fn run() {
    const SIZE: usize = 25;
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line|
        line.expect("read error").parse::<usize>().expect("invalid input")
    ).collect::<Vec<_>>();

    let mut result: Option<usize> = None;
    for window in input.windows(SIZE + 1) {
        let target = window.last().unwrap();
        let preceding = &window[0..SIZE];

        let found = preceding.iter().find(|&i|
            preceding.iter().find(|&j|
                i + j == *target
            ) != None
        ) != None;

        if !found {
            result = Some(*target);
            break;
        }
    }

    let target = result.expect("part 1 not found");
    for i in 0..input.len()-2 {
        for j in i+1..input.len() {
            let slice = &input[i..j];
            let sum: usize = slice.iter().sum();
            match sum.cmp(&target) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Greater => break,
                std::cmp::Ordering::Equal => {
                    println!("{}", slice.iter().min().unwrap() + slice.iter().max().unwrap());
                    return;
                }
            }
        }
    }
}
