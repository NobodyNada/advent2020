use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let start = lines.next().expect("unexpected EOF");
    let start = start.expect("read error").parse::<i32>().expect("invalid start time");

    let schedule = lines.next().expect("unexpected EOF").expect("read error");
    let schedule = schedule.split(',').filter_map(|entry|
        entry.parse::<i32>().ok()
    );

    let result = schedule.map(|entry|
        (entry, (start + entry-1)/entry * entry)
    ).min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    println!("{}", result.0 * (result.1 - start));
}
