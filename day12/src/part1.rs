use std::io::{self, BufRead};

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
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

    fn right(self, degrees: i32) -> Direction {
        let mut result = self;
        let mut degrees = degrees.rem_euclid(360);
        while degrees > 0 {
            result = match result {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North
            };
            degrees -= 90;
        }

        assert!(degrees == 0);
        result
    }

    fn left(self, degrees: i32) -> Direction {
        self.right(-degrees)
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
    let mut coords: (i32, i32) = (0, 0);
    let mut direction = Direction::East;
    for line in io::stdin().lock().lines() {
        let line = line.expect("read error");
        let (op, operand) = (line.bytes().next(), &line[1..]);
        let op = op.expect("empty line");
        let operand: i32 = operand.parse().expect("invalid input");

        match op {
            b'N' | b'S' | b'E' | b'W' => coords = Direction::from_char(op).unwrap().offset_coords_by(coords, operand),
            b'L' => direction = direction.left(operand),
            b'R' => direction = direction.right(operand),
            b'F' => coords = direction.offset_coords_by(coords, operand),
            _ => panic!("invalid operation '{}'", op)
        }
    }
    println!("{}", coords.0.abs() + coords.1.abs());
}
