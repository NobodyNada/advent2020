use std::io::{self, BufRead};
use std::collections::HashMap;

struct Bag {
    color: String,
    contents: Vec<(usize, String)>
}

impl Bag {
    fn parse(input: &str) -> Bag {
        lazy_static::lazy_static! {
            static ref OUTER: regex::Regex = regex::Regex::new(r"^(\w+ \w+) bags contain (.*)$").unwrap();
            static ref INNER: regex::Regex = regex::Regex::new(r"^\s*(\d+) (\w+ \w+) bags?(?:\s*|\.)$").unwrap();
        }

        let outer = OUTER.captures(&input).unwrap_or_else(|| panic!("invalid line {}", input));
        let color = outer.get(1).unwrap().as_str().to_string();
        let contents: Vec<(usize, String)> = outer.get(2).unwrap().as_str()
            .split(',').filter_map(|item|
                match item {
                    "no other bags." => None,
                    _ =>  {
                        let inner = INNER.captures(&item)
                            .unwrap_or_else(|| panic!("invalid item '{}' in line '{}'", item, input));
                        Some((inner.get(1).unwrap().as_str().parse::<usize>().unwrap(), // quantity 
                         inner.get(2).unwrap().as_str().to_string()))
                    }
                }
            ).collect();
        Bag { color, contents }
    }

    fn num_contained(&self, all_bags: &HashMap<String, Bag>) -> usize {
        self.contents.iter().fold(1, |accum, (count, color)|
            accum + count*all_bags.get(color).unwrap_or_else(|| panic!("color {} missing", color))
                .num_contained(&all_bags)
        )
    }
}

pub fn run() {
    let stdin = io::stdin();
    let bags: HashMap<String, Bag> = stdin.lock().lines().map(|line| {
        let bag = Bag::parse(&line.expect("read error"));
        (bag.color.clone(), bag)
    }).collect();
    // -1 because, once again, the shiny gold bag isn't included :/
    let result = bags.get("shiny gold").expect("no shiny gold bag").num_contained(&bags) - 1;
    println!("{}", result);
}
