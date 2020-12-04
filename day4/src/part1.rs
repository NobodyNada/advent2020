use std::io::{self, BufRead};
use std::collections::HashSet;

pub fn run() {
    let valid = io::stdin().lock().lines()
        .map(|line| line.expect("read error")).collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .filter(|passport| is_valid(passport)).count();
    println!("{}", valid);
}

pub fn is_valid(passport: &[String]) -> bool {
    // parse into list of (name, value)
    let passport = passport.join(" ");
    let fields = passport.split_whitespace()
        .map(|field| field.splitn(2, ":"))
        .map(|mut items| (items.nth(0).expect("invalid field"), items.nth(0).expect("invalid field")));

    // Required fields we haven't seen yet
    let mut required: HashSet<_> = [
        "byr", "iyr", "eyr", "hgt",
        "hcl", "ecl", "pid", 
    ].iter().collect();

    for (name, _) in fields {
        if !required.remove(&name) && name != "cid" { return false; }
    }
    required.is_empty()
}
