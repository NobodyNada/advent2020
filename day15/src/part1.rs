use std::io::{self, BufRead};
use std::iter;

/// An iterator which produces items in the elf sequence.
struct ElfIterator<S: Iterator<Item=u32>> {
    /// The remaining starting numbers.
    starting_numbers: iter::Fuse<S>,
    
    /// The number which will be spoken on the next turn.
    next: u32,

    /// The current turn number.
    turn: u32,

    /// The turn on which each number was previously spoken.
    spoken: std::collections::HashMap<u32, u32>,
}

impl<S: Iterator<Item=u32>> ElfIterator<S> {
    fn new(mut starting_numbers: S) -> Self {
        Self { 
            next: starting_numbers.next()
                .expect("starting_numbers must not be empty"),
            starting_numbers: starting_numbers.fuse(), 
            spoken: std::collections::HashMap::new(),
            turn: 0,
        }
    }
}

impl<S: Iterator<Item=u32>> Iterator for ElfIterator<S> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next;
        self.next = self.starting_numbers.next()
            .or_else(|| self.spoken.get(&result)
                .map(|prev_turn| self.turn - prev_turn))
            .unwrap_or(0);

        self.spoken.insert(result, self.turn);
        self.turn += 1;
        Some(result)
    }
}

pub fn run() {
    let stdin = io::stdin();
    let starting_numbers = 
        stdin.lock().split(b',').map(|item| 
            std::str::from_utf8(&item.expect("read error")[..]).expect("invalid input")
            .trim().parse::<u32>().expect("invalid input"));

    let mut sequence = ElfIterator::new(starting_numbers.fuse());
    println!("{}", sequence.nth(2019).unwrap());
}
