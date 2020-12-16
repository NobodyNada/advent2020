use std::io::{self, BufRead};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
struct TicketSchema {
    fields: Vec<(String, FieldSchema)>
}

impl TicketSchema {
    fn sum_invalid(&self, values: &[i32]) -> i32 {
        values.iter().filter(|&&value|
            self.fields.iter().find(|(_, field)| field.can_contain(value)).is_none()
        ).sum()
    }
}

#[derive(Debug, Clone)]
struct FieldSchema {
    ranges: Vec<std::ops::RangeInclusive<i32>>
}

impl FieldSchema {
    fn can_contain(&self, value: i32) -> bool {
        self.ranges.iter().find(|range| range.contains(&value)) != None
    }

    fn parse(text: &str) -> Option<(String, FieldSchema)> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new("^([^:]+): (.*)$").unwrap();
        }
        let captures = REGEX.captures(text)?;
        let name = captures.get(1).unwrap();
        let values = captures.get(2).unwrap().as_str()
            .split(" or ")
            .map(|range| {
               let mut split = range.splitn(2, '-');
               let lower = split.next()?.parse::<i32>().ok()?;
               let upper = split.next()?.parse::<i32>().ok()?;
               if lower <= upper { Some(lower..=upper) }
               else { dbg!(None) }
            }).collect::<Option<Vec<_>>>()?;
        
        Some((name.as_str().to_string(), FieldSchema { ranges: values }))
    }
}

pub fn run() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.expect("read error"));
    
    let schema = lines.by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| FieldSchema::parse(&line))
        .collect::<Option<Vec<(String, FieldSchema)>>>()
        .expect("could not parse schema");
    let schema = TicketSchema { fields: schema };

    // parse "your ticket" to /dev/null
    let _ = lines.by_ref()
        .take_while(|line| !line.is_empty())
        .for_each(std::mem::drop);

    let tickets =
        lines.by_ref()
        .skip(1) // skip "nearby tickets" line
        .take_while(|line| !line.is_empty())
        .map(|line|
            line.split(',').map(str::parse::<i32>)
                .collect::<Result<Vec<i32>,_>>()
                .expect("could not parse ticket")
        );

    println!("{}", tickets.map(|ticket| schema.sum_invalid(&ticket)).sum::<i32>());
}
