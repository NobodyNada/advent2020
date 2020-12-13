use std::io::{self, BufRead};

/// Solves a set of equations of the form
/// x mod M1 = A1
/// x mod M2 = A2
/// x mod M3 = A3
/// ...and so on.
/// elements is a boxed iterator over (A, N) pairs.
fn solve<'a>(elements: Box<dyn Iterator<Item=(u64, u64)> + 'a>) -> u64 {
    let mut elements = elements;
    
    // Base case: if we have zero equations, then
    // zero is the smallest solution in ℕ.
    let (first_offset, first_mod) = match elements.next() {
        Some((a, m)) => (a, m),
        None => return 0
    };
    // first_offset and first_mod are M1 and A1, respectively

    // The solutions to the first equation are of the form x = n*first_mod + first_offset.
    // The solutions to the remaining equations will "cycle": for an equation with modulus N,
    // every Nth solution to the first equation will also be a solution to this equation.
    // This is most clearly illustrated with an example. Consider the following set of equations:
    // x mod 5 = 0
    // x mod 7 = 1
    // Every 5th integer is a solution to the first equation: 0, 5, 10, etc. Out of these
    // solutions, every 7th is also a solution to the second equation: 15, 50, 85, etc.
    // 
    // We can express the second equation in terms of the *n* of the first equation:
    // n mod 7 = 3, which has solutions n = 3, 10, 17, etc.
    //
    // Crucially, this equation is also of the form we're trying to solve -- so we can just rewrite
    // all equations except the first one in terms of the *n* of the first one, and then
    // recursively solve the new set of equations.

    let remaining = elements.map(|(offset, modulus)| {
        // Find the smallest new_offset such that the new_offset'th solution to the first equation
        // is a valid solution to this equation.
        let new_offset = (0..modulus).find(|new_offset|
            (first_offset + new_offset*first_mod) % modulus == offset
        ).expect("no solution");

        // The rewritten equation is n mod <modulus> = <new_offset>.
        (new_offset, modulus)
    });

    // Recursively solve for n.
    let n = solve(Box::new(remaining));

    // Substitute it into the original equation.
    n*first_mod + first_offset
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

    let solution = solve(Box::new(schedule.iter().cloned()));
    println!("{}", solution);
    
    // Verify the solution is valid.
    schedule.iter().for_each(|&(offset, modulus)|
        assert!(solution % modulus == offset)
    );
}
