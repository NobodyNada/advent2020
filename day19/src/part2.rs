use std::io::{self, BufRead};
use std::collections::HashMap;

enum Rule {
    /// This rule matches a literal character.
    Verbatim(u8),

    /// This rule matches a disjunction of several rule sequences.
    Disjunction(Vec<Vec<usize>>)
}

impl Rule {
    fn parse(line: &str) -> Option<(usize, Rule)> {
        let mut splits = line.splitn(2, ':');
        let index: usize = splits.next()?.parse().ok()?;
        let rule = splits.next()?.trim();

        let mut chars = rule.bytes();
        if chars.next() == Some(b'"') { Some((index, Rule::Verbatim(chars.next()?))) }
        else {
            let alternatives = 
                rule.split('|').map(|alternative|
                    alternative.trim().split_whitespace()
                        .map(|requirement| requirement.parse())
                        .collect::<Result<Vec<usize>, _>>()
                ).collect::<Result<Vec<Vec<usize>>, _>>().ok()?;
            Some((index, Rule::Disjunction(alternatives)))
        }
    }

    /// Evaluates a rule starting at the first character of the given input.
    /// Returns the remaining characters after each possible match. For example:
    /// ```
    /// // rule = "ab" | "aba"
    /// rule.evaluate("abc")  -> [ "c" ]
    /// rule.evaluate("ab")   -> [ "" ]
    /// rule.evaluate("abad") -> [ "ad",  "d" ]
    /// rule.evaluate("aba")  -> [ "a",   ""  ] 
    /// rule.evaluate("bad")  -> [  ]
    /// ```
    fn evaluate<'a>(&self, input: &'a str, ruleset: &HashMap<usize, Rule>) -> Vec<&'a str> {
        // We can't possibly match on no characters. This stops us from recursing infinitely.
        if input.is_empty() { return Vec::new(); }

        match self {
            Rule::Verbatim(c) => 
                if input.bytes().next() == Some(*c) {
                    vec![&input[1..]]
                } else {
                    Vec::new()
                },
            Rule::Disjunction(alternatives) => alternatives.iter().flat_map(|sequence| {
                // Start with the single input
                let mut inputs = vec![input];

                // All of the rules in the sequence must match.
                for rule in sequence {
                    let rule = ruleset.get(rule).expect("missing rule");

                    // Evaluate the next rule in the sequence on each partial match,
                    // transforming each partial match into zero or more further matches.
                    inputs = inputs.iter()
                        .flat_map(|input| rule.evaluate(input, ruleset))
                        .collect();
                }
                inputs
            }).collect()
        }
    }
}

pub fn run() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines()
        .map(|line| line.expect("read error"));

    let mut rules = lines.by_ref().take_while(|line| !line.is_empty())
        .map(|line| Rule::parse(&line))
        .collect::<Option<HashMap<usize, _>>>().expect("invalid input");

    rules.insert(8, Rule::Disjunction(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Disjunction(vec![vec![42, 31], vec![42, 11, 31]]));

    let inputs = lines;

    let first_rule = rules.get(&0).expect("no rule 0");
    let matching = inputs.filter(|input| {
        first_rule.evaluate(&input, &rules).iter().find(|remainder| remainder.is_empty()) != None
    });
    println!("{}", matching.count());
}
