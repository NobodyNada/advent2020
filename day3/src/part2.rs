use std::io::{self, BufRead};

pub fn run() {
    // The horizontal position on the map.
    let lines = io::stdin().lock().lines()
        .map(|line| line.expect("read error")).collect();
    let trees = [
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2)
    ].iter()
        .map(|(x, y)| check_slope(&lines, *x, *y))
        .fold(1, |accum, next| accum*next);

    println!("{:?}", trees);
}

fn check_slope(lines: &Vec<String>, x_slope: usize, y_slope: usize) -> usize {
    // The current horizontal position.
    let mut x_pos = 0;

    // The number of lines to skip vertically before the next one we check.
    let mut y_skip = 0;

    let mut trees = 0;
    let mut line_length: Option<usize> = None;

    for line in lines {
        if let Some(length) = line_length {
            assert_eq!(length, line.len(), "mismatched line lengths");
        } else {
            line_length = Some(line.len());
        }

        if y_skip == 0 {
            if line.as_bytes()[x_pos as usize] == '#' as u8 { trees += 1; }
            x_pos = (x_pos + x_slope) % line.len() as usize;
            y_skip = y_slope;
        }
        y_skip -= 1;
    }

    trees
}
