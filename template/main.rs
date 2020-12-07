mod part1;
mod part2;

fn main() {
    match std::env::args().nth(1).map(|arg| arg.parse::<i32>()) {
        Some(Ok(1)) => part1::run(),
        Some(Ok(2)) => part2::run(),
        _ => panic!("usage: {} 1|2", std::env::args().next().unwrap())
    }
}
