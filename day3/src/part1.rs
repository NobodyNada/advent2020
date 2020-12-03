use std::io::{self, BufRead};

pub fn run() {
    // The horizontal position on the map.
    let mut x_pos = 0;
    let mut trees = 0;
    let mut line_length: Option<usize> = None;
    for line in io::stdin().lock().lines() {
        let line = line.expect("read error");

        if let Some(length) = line_length {
            assert_eq!(length, line.len(), "mismatched line lengths");
        } else {
            line_length = Some(line.len());
        }

        if line.as_bytes()[x_pos] == '#' as u8 { trees += 1; }
        x_pos = (x_pos + 3) % line.len();
    }
    println!("{}", trees);
}
