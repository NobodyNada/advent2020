use std::io::{self, BufRead};
use std::collections::HashMap;

pub fn run() {
    let mut adapters: Vec<u32> = std::iter::once(0).chain(
        io::stdin().lock().lines()
            .map(|line| line.expect("read error").parse::<u32>().expect("parse error"))
    ).collect();

    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);

    let differences = adapters.windows(2).map(|window| match window {
        [a, b] => b-a,
        _ => panic!(".windows(2) always returns slices of two elements")
    });
    let mut histogram = HashMap::<u32, u32>::new();
    differences.for_each(|difference|
        *histogram.entry(difference).or_insert(0) += 1
    );

    println!("{:?}", histogram);
}
