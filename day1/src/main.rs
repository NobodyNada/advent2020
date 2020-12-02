use std::io::{self, BufRead};

fn main() {
    // The sum we're looking for.
    const TARGET: usize = 2020;

    // Values which we've encountered in the input so far.
    let mut seen = vec![false; TARGET];
    
    for line in io::stdin().lock().lines() {
        let value = line.expect("read error")
            .parse::<usize>().expect("invalid integer");
        assert!(value <= TARGET, "value {} out of range", value);

        seen[value] = true;
        let other = TARGET - value;
        if seen[other] {
            println!("{}", value * other);
            break;
        }
    }
}
