use std::io::{self, BufRead};

pub fn run() {
    let count = io::stdin().lock().lines()
        .filter(|line|
            is_valid(&line.as_ref().expect("read error"))
        ).count();
    println!("{}", count);
}

fn is_valid(input: &str) -> bool { 
    // use nested function to simplify error handling
    (|| {
        // password will have a leading space, but that's fine
        let (policy, password) = partition(input, ':')?;

        let (occurences, letter) = partition(policy, ' ')?;
        assert!(letter.chars().count() == 1, "{} is not a single character", letter);
        let letter = letter.chars().next()?;

        let (min, max) = partition(occurences, '-')?;
        let (min, max): (usize, usize) = (
            min.parse().ok()?,
            max.parse().ok()?
        );

        let count = password.chars().filter(|c| c == &letter).count();
        Some(count >= min && count <= max)
    })().expect(&format!("invalid input {}", input))
}

/// Divides a string into exactly two halves separated by a character.
fn partition(input: &str, separator: char) -> Option<(&str, &str)> {
    let mut split = input.splitn(2, separator);
    match (split.next(), split.next()) {
        (Some(first), Some(second)) => Some((first, second)),
        _ => None
    }
}
