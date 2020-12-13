use std::io::{self, BufRead};

// args are (offset, modulus) pairs
fn solve(elements: impl Iterator<Item=(u64, u64)>) -> u64 {
    let mut elements = elements;
    let (first_offset, first_mod) = match elements.next() {
        Some(a) => dbg!(a),
        None => return dbg!(0)
    };

    let remaining = elements.map(|(offset, modulus)| {
        let start = dbg!(first_offset);
        let new_offset = (0..modulus).find(|test|
            (start + test*first_mod) % modulus == offset
        ).expect("no solution");
        dbg!((new_offset, modulus))
    });

    dbg!(solve(remaining.collect::<Vec<_>>().into_iter())) * dbg!(first_mod) + dbg!(first_offset)
}

pub fn run() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let schedule = lines.nth(1).expect("unexpected EOF").expect("read error")
        .split(',').enumerate().filter_map(|(index, entry)| {
            let (index, entry) = (index as i32, entry.parse::<i32>().ok()?);
            let index = (-index).rem_euclid(entry);
            Some((index as u64, entry as u64))
        }).collect::<Vec<_>>();

    let solution = dbg!(solve(schedule.clone().into_iter()));
    schedule.iter().for_each(|&(offset, modulus)|
        assert!(solution % modulus == offset)
    );
}
