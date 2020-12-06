use std::io::{self, BufRead};
use std::collections::HashSet;

pub fn run() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.expect("read error")).collect::<Vec<_>>();
    let groups = lines.split(|line| line.is_empty());
    let result: usize = groups.map(count_yes).sum();
    println!("{}", result)
}

/// Returns the number of questions for which anyone in the group answers "yes".
fn count_yes(group: &[String]) -> usize {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut set: HashSet<_> = ALPHABET.chars().collect();
    group.iter().for_each(|line| {
        for c in ALPHABET.chars() {
            if !line.contains(c) { set.remove(&c); }
        }
    });
    set.len()
}
