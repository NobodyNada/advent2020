use std::io::{self, BufRead};
use lazy_static::lazy_static;
use regex::Regex;

const TILE_SIZE: usize = 10;

/// Represents the border of a tile. Elements run
/// left-to-right or top-to-bottom.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Border {
    raw: u16
}

impl Border {
    fn parse(chars: impl Iterator<Item=u8>) -> Option<Border> {
        chars.enumerate().try_fold(0, |accum, (index, next)| {
            if index >= TILE_SIZE { return None; }
            let bit = match next {
                b'#' => 1,
                b'.' => 0,
                _ => return None
            };
            Some(accum << 1 | bit)
        }).map(Border::from_raw)
    }

    fn from_raw(raw: u16) -> Border {
        assert!(raw < 1 << TILE_SIZE);
        Border { raw }
    }

    fn flipped(&self) -> Border {
        Border::from_raw(self.raw.reverse_bits() >> (16 - TILE_SIZE))
    }
}

#[derive(Debug)]
struct Tile {
    id: u32,
    
    /// The edges.
    border: [Border; 4]
}

impl Tile {
    fn parse(input: &[String]) -> Option<Tile> {
        if input.len() != TILE_SIZE + 1 { return None; }

        let id_line = &input[0];
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
        }
        let tile_id = REGEX
            .captures(&id_line)?.get(1)?
            .as_str().parse().ok()?;

        let input = &input[1..];
        if input.iter().find(|line| line.len() != TILE_SIZE) != None { return None; }
        Some(Tile {
            id: tile_id,
            border: [
                Border::parse(input.first()?.bytes())?,
                Border::parse(input.iter().map(|line| line.bytes().last().unwrap()))?,
                Border::parse(input.last()?.bytes())?,
                Border::parse(input.iter().map(|line| line.bytes().next().unwrap()))?
            ]
        })
    }
}

pub fn run() {
    let stdin = io::stdin();
    let tiles = stdin.lock().lines()
        .map(|line| line.expect("read error")).collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .filter(|split| !split.is_empty())
        .map(|tile| Tile::parse(&tile).expect("invalid input"))
        .collect::<Vec<_>>();

    let corners = tiles.iter().filter(|tile| {
        // for each border...
        let matches = tile.border.iter().cloned().flat_map(|border| {
            // for each other tile...
            tiles.iter().filter(|other_tile| other_tile.id != tile.id)
                .zip(std::iter::repeat(border)).flat_map(|(other_tile, border)| 
                    // for each other border...
                    other_tile.border.iter().filter(move |other_border|
                        border.raw == other_border.raw || border.flipped().raw == other_border.raw
                ).map(move |_| other_tile)
        )});

        // corners are adjacent to exactly 2 other tiles
        matches.count() == 2
    }).collect::<Vec<_>>();

    assert_eq!(corners.len(), 4);
    println!("{}", corners.iter().map(|tile| tile.id as u64).product::<u64>())
}
