use std::io::{self, BufRead};

#[derive(Copy, Clone)]
enum Direction {
    North, South,
    East,
    West
}

impl Direction {
    fn from_char(c: u8) -> Option<Direction> {
        match c {
            b'N' => Some(Direction::North),
            b'S' => Some(Direction::South),
            b'E' => Some(Direction::East),
            b'W' => Some(Direction::West),
            _ => None
        }
    }

    fn dxy(self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::South => (0, -1)
        }
    }

    fn offset_coords_by(self, coords: (i32, i32), distance: i32) -> (i32, i32) {
        (
            coords.0 + self.dxy().0*distance,
            coords.1 + self.dxy().1*distance
        )
    }
}

pub fn run() {
    // The current ship location.
    let mut coords: (i32, i32) = (0, 0);

    // The location of the waypoint relative to the ship.
    let mut wpt_offset: (i32, i32) = (10, 1);
    for line in io::stdin().lock().lines() {
        let line = line.expect("read error");
        let (op, operand) = (line.bytes().next(), &line[1..]);
        let op = op.expect("empty line");
        let operand: i32 = operand.parse().expect("invalid input");

        match op {
            b'N' | b'S' | b'E' | b'W' => wpt_offset = Direction::from_char(op).unwrap().offset_coords_by(wpt_offset, operand),
            b'L' => (0..operand/90).for_each(|_| wpt_offset = (
                -wpt_offset.1,
                wpt_offset.0
            )),
            b'R' => (0..operand/90).for_each(|_| wpt_offset = (
                wpt_offset.1,
                -wpt_offset.0
            )),
            b'F' => coords = (
                coords.0 + wpt_offset.0*operand,
                coords.1 + wpt_offset.1*operand
            ),
            _ => panic!("invalid operation")
        }
    }
    println!("{}", coords.0.abs() + coords.1.abs());
}
