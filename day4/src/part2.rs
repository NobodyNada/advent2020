use std::io::{self, BufRead};
use std::collections::HashMap;

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

    // Fields and their validators
    type Validator = fn(&str) -> bool;
    let required: Vec<(&str, Validator)> = vec![
        ("byr", |field: &str| field.len() == 4 && (1920..=2002).contains(&field.parse().unwrap())),
        ("iyr", |field: &str| field.len() == 4 && (2010..=2020).contains(&field.parse().unwrap())),
        ("eyr", |field: &str| field.len() == 4 && (2020..=2030).contains(&field.parse().unwrap())),
        ("hgt", |field: &str| match (&field[0..field.len()-2].parse::<i32>(), &field[field.len()-2..]) {
            (Err(_), _) => false,
            (Ok(150..=193), "cm") => true,
            (Ok(59..=76), "in") => true,
            _ => false
        }),
        ("hcl", |field: &str| field.len() == 7 && {
            let mut bytes = field.bytes();
            // String starts with # and only contains 0-9, a-f
            bytes.nth(0) == Some(b'#') && 
                bytes.filter(|c| "0123456789abcdef".bytes().find(|x| x == c).is_none()).nth(0).is_none()
        }),
        ("ecl", |field: &str| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&field)),
        ("pid", |field: &str| field.len() == 9 && field.parse::<u32>().is_ok()),
    ];
    let mut required: HashMap<_, _> = required.into_iter().collect();

    for (name, value) in fields {
        match required.remove(&name) {
            Some(validator) => if !validator(&value) { println!("{}:{}", name, value); return false; }
            None => if name != "cid" { return false; }
        }
    }
    required.is_empty()
}
