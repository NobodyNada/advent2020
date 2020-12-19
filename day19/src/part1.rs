use std::io::{self, BufRead};
use std::collections::HashMap;

enum Rule {
    Verbatim(u8),
    Ruleset(Vec<Vec<usize>>)
}

impl Rule {
    fn parse(line: &str) -> Option<(usize, Rule)> {
        let mut splits = line.splitn(2, ':');
        let index: usize = splits.next()?.parse().ok()?;
        let rule = splits.next()?.trim();

        let mut chars = rule.bytes();
        if chars.next() == Some(b'"') { Some((index, Rule::Verbatim(chars.next()?))) }
        else {
            let disjunctions = 
                rule.split('|').map(|alternative|
                    alternative.trim().split_whitespace()
                        .map(|requirement| requirement.parse())
                        .collect::<Result<Vec<usize>, _>>()
                ).collect::<Result<Vec<Vec<usize>>, _>>().ok()?;
            Some((index, Rule::Ruleset(disjunctions)))
        }
    }

    fn evaluate<'a>(&self, input: &'a str, ruleset: &HashMap<usize, Rule>) -> Option<&'a str> {
        match self {
            Rule::Verbatim(c) => 
                if input.bytes().next() == Some(*c) {
                    Some(&input[1..])
                } else {
                    None
                },
            Rule::Ruleset(disjunctions) => disjunctions.iter()
                .filter_map(|alternative| {
                    let mut input = input;

                    for rule in alternative {
                        input = ruleset.get(rule).and_then(|rule| rule.evaluate(input, ruleset))?;
                    }
                    Some(input)
                }).next()
        }
    }
}

pub fn run() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines()
        .map(|line| line.expect("read error"));

    let rules = lines.by_ref().take_while(|line| !line.is_empty())
        .map(|line| Rule::parse(&line))
        .collect::<Option<HashMap<usize, _>>>().expect("invalid input");

    let inputs = lines;

    let first_rule = rules.get(&0).expect("no rule 0");
    let matching = inputs.filter(|input| first_rule.evaluate(input, &rules)
        .map(|remainder| remainder.is_empty()).unwrap_or(false)
    );
    println!("{}", matching.count());
}
