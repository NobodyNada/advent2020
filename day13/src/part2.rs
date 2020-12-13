use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let schedule = lines.nth(1).expect("unexpected EOF").expect("read error");
    let schedule = schedule.split(',').map(|entry|
        entry.parse::<u64>().ok()
    );
    let schedule = schedule.collect::<Vec<_>>();
    let (biggest_offset, biggest) = schedule.iter().enumerate().filter_map(|(a, &b)| Some((a, b?))).max_by(|a, b|
        a.1.cmp(&b.1)
    ).unwrap();

    for i in 1.. {
        let time = i*biggest - biggest_offset as u64;
        let mut found = true;
        for (offset, bus) in schedule.iter().enumerate() {
            if let Some(bus) = bus {
                if (time + offset as u64) % bus != 0 {
                    found = false;
                    break;
                }
            }
        }

        if found {
            // we found it!
            println!("{}", time);
            break;
        }
    }
}
