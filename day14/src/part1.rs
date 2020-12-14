use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Mask {
    and: u64,
    or: u64
}

impl Mask {
    fn parse(input: &str) -> Option<Mask> {
        let mut and = 0;
        let mut or = 0;
        for c in input.bytes() {
            match c {
                b'0' => {
                    and <<= 1;
                    or <<= 1;
                },
                b'1' => {
                    and <<= 1;
                    or = (or << 1) | 1;
                },
                b'X' => {
                    and = (and << 1) | 1;
                    or <<= 1;
                }
                _ => return None
            };
        }

        Some(Mask { and, or })
    }

    fn apply(&self, value: u64) -> u64 {
        (value & self.and) | self.or
    }
}

pub fn run() {
    let mask_regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    let mut current_mask = Mask { and: 0, or: 0 };
    let mut memory = std::collections::HashMap::<u64, u64>::new();

    for line in io::stdin().lock().lines() {
        let line = line.expect("read error");

        if let Some(mask) = mask_regex.captures_iter(&line).next() {
            let mask = &mask[1];
            current_mask = Mask::parse(mask).unwrap_or_else(|| panic!("invalid mask {}", mask));
        } else if let Some(write) = mem_regex.captures_iter(&line).next() {
            let addr = write[1].parse::<u64>().expect("invalid addr");
            let val = write[2].parse::<u64>().expect("invalid val");
            memory.insert(addr, current_mask.apply(val));
        } else {
            panic!("invalid input");
        }
    }

    println!("{}", memory.iter().map(|(_, &val)| val).sum::<u64>());
}
