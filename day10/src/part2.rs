use std::{cell::Cell, io::{self, BufRead}, str::FromStr};

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone)]
struct Adapter { 
    joltage: i32,
    chains: Cell<Option<usize>> // A memoized number of chains
}

impl FromStr for Adapter {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?))
    }
}

impl Adapter {
    fn new(joltage: i32) -> Self {
        Self { joltage, chains: Cell::new(None) }
    }
    fn can_connect_after(&self, other: &Adapter) -> bool {
        (1..=3).contains(&(self.joltage - other.joltage))
    }

    fn num_chains(adapters: &[Adapter]) -> usize {
        // Base case: we have exactly zero or one adapters in the chain
        if adapters.len() <= 1 { return 1; }

        // General case: recursively count the number of chains
        // that can be formed using the given set of adapters.

        // Get the first adapter in the chain
        let start = adapters.first().unwrap();

        // If we already know how many chains can be formed with these
        // adapters, return the memoized value instead of computing it again
        if let Some(memoized) = start.chains.get() { return memoized; }

        let mut remaining = &adapters[1..];
        let mut result = 0;
        while let Some(next) = remaining.first() {
            if next.can_connect_after(start) {
                // We can chain from start to next, so try
                // building chains from next to the rest of adapters.
                result += Adapter::num_chains(remaining);

                // Loop and see if we can build more chains by skipping this adapter.
                remaining = &remaining[1..];
            } else {
                // We can't chain from start to next. Adapters is sorted, so we know
                // we can't possibly build any more chains starting with start.
                break;
            }
        }

        // Memoize and return the result
        start.chains.set(Some(result));
        result
    }
}

pub fn run() {
    let mut adapters: Vec<Adapter> = std::iter::once(Adapter::new(0))
        .chain(
            io::stdin().lock().lines()
                .map(|line| line.expect("read error").parse::<Adapter>().expect("parse error"))
    ).collect();

    adapters.sort_unstable();
    adapters.push(Adapter::new(adapters.last().unwrap().joltage + 3));

    println!("{}", Adapter::num_chains(&adapters));
}
