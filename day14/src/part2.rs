use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Mask {
    and: u64,
    or: u64
}

impl Mask {
    fn parse(input: &str) -> Option<Vec<Mask>> {
        let mut masks = vec![Mask { and: 0, or: 0 }];
        for c in input.bytes() {
            match c {
                b'0' => {
                    masks.iter_mut().for_each(|mask| {
                        mask.and = (mask.and << 1) | 1;
                        mask.or <<= 1;
                    })
                },
                b'1' => {
                    masks.iter_mut().for_each(|mask| {
                        mask.and <<= 1;
                        mask.or = (mask.or << 1) | 1;
                    })
                },
                b'X' => {
                    masks.reserve(masks.len());
                    for i in 0..masks.len() {
                        let mask = masks[i];
                        masks[i] = Mask { and: mask.and << 1, or: mask.or << 1 };
                        masks.push(Mask { and: mask.and << 1, or: (mask.or << 1) | 1 });
                    }
                }
                _ => return None
            };
        }

        Some(masks)
    }

    fn apply<'a>(masks: &'a [Mask], value: u64) -> impl Iterator<Item=u64> + 'a {
        masks.iter().map(move |mask|
            (value & mask.and) | mask.or
        )
    }
}

pub fn run() {
    let mask_regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    let mut current_masks = Vec::new();
    let mut memory = std::collections::HashMap::<u64, u64>::new();

    for line in io::stdin().lock().lines() {
        let line = line.expect("read error");

        if let Some(mask) = mask_regex.captures_iter(&line).next() {
            let mask = &mask[1];
            current_masks = Mask::parse(mask).unwrap_or_else(|| panic!("invalid mask {}", mask));
        } else if let Some(write) = mem_regex.captures_iter(&line).next() {
            let addr = write[1].parse::<u64>().expect("invalid addr");
            let val = write[2].parse::<u64>().expect("invalid val");
            Mask::apply(&current_masks, addr).for_each(|addr|
                { memory.insert(addr, val); }
            );
        } else {
            panic!("invalid input");
        }
    }

    println!("{}", memory.iter().map(|(_, &val)| val).sum::<u64>());
}
