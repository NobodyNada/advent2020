use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
struct TicketSchema {
    fields: Vec<(String, FieldSchema)>
}

impl TicketSchema {
    fn is_valid(&self, values: &[i32]) -> bool {
        let mut invalid = values.iter().filter(|&&value|
            self.fields.iter().find(|(_, field)| field.can_contain(value)).is_none()
        );
        invalid.next() == None
    }

    // Determines the index of each field.
    fn solve(&mut self, tickets: impl Iterator<Item=Vec<i32>>) -> Option<HashMap<String, usize>> {
        // 1st pass: eliminate trivially-invalid values
        for ticket in tickets {
            for (index, &value) in ticket.iter().enumerate() {
                self.fields.iter_mut().filter(|(_, field)| !field.can_contain(value))
                    .for_each(|(_, field)| {field.possible_indices.remove(&index);});
            }
        }

        // 2nd-nth passes: if we have tickets with only one possible index,
        // remove that index from other tickets
        let mut known = HashMap::new();
        let mut unknown = self.fields.clone();
        while !unknown.is_empty() {
            let mut found = false;

            for i in 0..unknown.len() {
                let (name, field) = unknown.get(i).unwrap();
                if field.possible_indices.len() == 1 {
                    // There is only one possible index for this field, so
                    // move it to known.
                    let index = *field.possible_indices.iter().next().unwrap();
                    known.insert(name.clone(), index);
                    unknown.remove(i);

                    // Since the index belongs to this field, it cannot
                    // possibly belong to any other field.
                    unknown.iter_mut().for_each(|(_, field)| {
                        field.possible_indices.remove(&index);
                    });

                    // We found one, so start the loop again.
                    found = true;
                    break;
                }
            }

            // If we made it a whole iteration without finding anything, give up.
            if !found { 
                return None;
            }
        }
        Some(known)
    }
}

#[derive(Debug, Clone)]
struct FieldSchema {
    ranges: Vec<std::ops::RangeInclusive<i32>>,
    possible_indices: HashSet<usize>,
}

impl FieldSchema {
    fn can_contain(&self, value: i32) -> bool {
        self.ranges.iter().find(|range| range.contains(&value)) != None
    }

    fn parse(text: &str, num_fields: usize) -> Option<(String, FieldSchema)> {
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
        
        Some((name.as_str().to_string(), FieldSchema { 
            ranges: values, possible_indices: (0..num_fields).collect()
        }))
    }
}

fn parse_ticket(line: &str) -> Option<Vec<i32>> {
    line.split(',').map(str::parse::<i32>)
        .collect::<Result<Vec<i32>,_>>()
        .ok()
}

#[allow(clippy::clippy::needless_collect)]
// Clippy is wrong about the "needless" collect of 'tickets';
// he doesn't recognize that the filter call borrows schema.
// https://github.com/rust-lang/rust-clippy/issues/6066
pub fn run() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.expect("read error"));
    
    let schema = lines.by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    // parse "your ticket"
    let mut mine_section = lines.by_ref()
        .take_while(|line| !line.is_empty());
    let mine = mine_section.nth(1) // skip "your ticket" line
        .and_then(|line| parse_ticket(&line))
        .expect("invalid ticket");
    mine_section.for_each(std::mem::drop);

    let num_fields = mine.len();

    // now that we know how many fields per ticket,
    // finish parsing the schema
    let schema = schema.into_iter()
        .map(|line| FieldSchema::parse(&line, num_fields))
        .collect::<Option<Vec<(String, FieldSchema)>>>()
        .expect("could not parse schema");
    let mut schema = TicketSchema { fields: schema };

    let tickets =
        lines.by_ref()
        .skip(1) // skip "nearby tickets" line
        .take_while(|line| !line.is_empty())
        .map(|line| parse_ticket(&line).expect("invalid ticket"))
        .filter(|ticket| schema.is_valid(ticket))
        .collect::<Vec<_>>();

    let schema = schema.solve(tickets.into_iter())
        .expect("no solution");
    
    let result = schema.iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, &index)| mine[index] as u64)
        .product::<u64>();
    println!("{}", result);
}
