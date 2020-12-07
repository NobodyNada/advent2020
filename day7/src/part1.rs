use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

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

    /// Checks whether this bag can contain the target, using the provided set of requirements.
    fn can_contain(&self, target: &str, all_bags: &HashMap<String, Bag>) -> bool {
        self.can_contain_recursive(target, all_bags, &mut HashSet::new())
    }

    fn can_contain_recursive(&self, target: &str, all_bags: &HashMap<String, Bag>, checked: &mut HashSet<String>) -> bool {
        // Do a depth-first search, skipping bags we've already checked.
        if self.color == target { true }
        else if !checked.insert(self.color.clone()) { false } // We've already checked this one. 
        else {
            let found = self.contents.iter().find(|(_, color)|
                all_bags.get(color).unwrap_or_else(|| panic!("unknown bag {}", color))
                    .can_contain_recursive(target, all_bags, checked)
            );
            checked.remove(&self.color);
            found != None
        }
    }
}

pub fn run() {
    let stdin = io::stdin();
    let bags: HashMap<String, Bag> = stdin.lock().lines().map(|line| {
        let bag = Bag::parse(&line.expect("read error"));
        (bag.color.clone(), bag)
    }).collect();
    let result = bags.iter().filter(|(_, bag)| {
        bag.can_contain("shiny gold", &bags)
    }).count() - 1; // -1 because "shiny gold" itself doesn't count
    println!("{}", result);
}
