use std::collections;
use std::io::{self, BufRead};

pub fn run() {
    // The sum we're looking for.
    const TARGET: i32 = 2020;

    // Values which we've encountered in the input so far.
    let mut seen = collections::HashSet::<i32>::new();
    
    for line in io::stdin().lock().lines() {
        let value = line.expect("read error")
            .parse::<i32>().expect("invalid integer");
        assert!(value <= TARGET, "value {} out of range", value);

        // For each pair of numbers we've seen so far...
        for other in &seen {
            // Compute the third value we need to complete the sum.
            let expected = TARGET - other - value;
            // Do we have this value?
            if seen.contains(&expected) {
                println!("{}", value * other * expected);
                break;
            }
        }

        // We still haven't found the sum, add this value to the list and loop.
        seen.insert(value);
    }
}
