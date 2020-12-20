use std::io::{self, BufRead};
use std::{cell::RefCell, collections::HashMap};
use lazy_static::lazy_static;
use regex::Regex;

const TILE_SIZE: usize = 10;
const CONTENT_SIZE: usize = TILE_SIZE-2;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Edge {
    Top, Right, Bottom, Left
}

impl Edge {
    fn index(self) -> usize {
        match self {
            Edge::Top => 0,
            Edge::Right => 1,
            Edge::Bottom => 2,
            Edge::Left => 3
        }
    }

    fn from_index(index: usize) -> Edge {
        match index {
            0 => Edge::Top,
            1 => Edge::Right,
            2 => Edge::Bottom,
            3 => Edge::Left,
            _ => panic!("edge index out of range")
        }
    }

    fn advanced(self, n: isize) -> Edge {
        Edge::from_index((self.index() as isize + n).rem_euclid(4) as usize)
    }

    fn inverse(self) -> Edge {
        self.advanced(2)
    }
}

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
    border: [Border; 4],

    content: [u16; CONTENT_SIZE],

    memoized_matches: RefCell<Option<[Option<Match>; 4]>>
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

        let mut content = [0; CONTENT_SIZE];
        for i in 0..CONTENT_SIZE {
            content[i] = input[i+1][1..(CONTENT_SIZE+1)].bytes().try_fold(0, |accum, next|
                Some(accum << 1 | match next {
                    b'#' => 1,
                    b'.' => 0,
                    _ => return None
                })
            )?;
        }

        Some(Tile {
            id: tile_id,
            border: [
                Border::parse(input.first()?.bytes())?,
                Border::parse(input.iter().map(|line| line.bytes().last().unwrap()))?,
                Border::parse(input.last()?.bytes())?,
                Border::parse(input.iter().map(|line| line.bytes().next().unwrap()))?
            ],
            content,
            memoized_matches: RefCell::new(None)
        })
    }

    fn flip_horiz(&mut self) {
        let new_border = [
            self.border[Edge::Top.index()].flipped(),
            self.border[Edge::Left.index()],
            self.border[Edge::Bottom.index()].flipped(),
            self.border[Edge::Right.index()]
        ];
        self.border = new_border;

        self.content.iter_mut().for_each(|row|
            *row = row.reverse_bits() >> (16 - CONTENT_SIZE)
        );

        let old_matches = self.get_memoized();
        let new_matches = old_matches.map(|old| [
            old[Edge::Top.index()].map(Match::flipped),
            old[Edge::Left.index()],
            old[Edge::Bottom.index()].map(Match::flipped),
            old[Edge::Right.index()]
        ]);
        self.memoized_matches.replace(new_matches);
    }

    fn flip_vert(&mut self) {
        let new_border = [
            self.border[Edge::Bottom.index()],
            self.border[Edge::Right.index()].flipped(),
            self.border[Edge::Top.index()],
            self.border[Edge::Left.index()].flipped()
        ];
        self.border = new_border;
        
        self.content.reverse();

        let old_matches = self.get_memoized();
        let new_matches = old_matches.map(|old| [
            old[Edge::Bottom.index()],
            old[Edge::Right.index()].map(Match::flipped),
            old[Edge::Top.index()],
            old[Edge::Left.index()].map(Match::flipped)
        ]);
        self.memoized_matches.replace(new_matches);
    }

    fn rotate_cw(&mut self) {
        let new_border = [
            self.border[Edge::Left.index()].flipped(),
            self.border[Edge::Top.index()],
            self.border[Edge::Right.index()].flipped(),
            self.border[Edge::Bottom.index()],
        ];
        self.border = new_border;

        let mut new_content = [0; CONTENT_SIZE];
        // new_content[row][col] = content[-col][row];
        for col in 0..CONTENT_SIZE {
            let mut old = self.content[CONTENT_SIZE - col - 1];
            for row in (0..CONTENT_SIZE).rev() {
                let bit = old & 1;
                old >>= 1;
                new_content[row] = (new_content[row] << 1) | bit;
            }
        }
        self.content = new_content;

        let old_matches = self.get_memoized();
        let new_matches = old_matches.map(|old| [
            old[Edge::Left.index()].map(Match::flipped),
            old[Edge::Top.index()],
            old[Edge::Right.index()].map(Match::flipped),
            old[Edge::Bottom.index()]
        ]);
        self.memoized_matches.replace(new_matches);
    }

    fn get_memoized(&self) -> Option<std::cell::Ref<'_, [Option<Match>; 4]>> {
        let memoized = self.memoized_matches.borrow();
        if memoized.is_some() {
            Some(std::cell::Ref::map(memoized, |m| m.as_ref().unwrap()))
        } else {
            None
        }
    }

    /// Iterates over the tiles matching each border.
    /// Note: May return a memoized value, which may not be correct if the
    /// tileset has been modified. Use search_matches instead to force a search.
    fn matches(&self, tiles: &HashMap<u32, Tile>)
    -> std::cell::Ref<'_, [Option<Match>; 4]> {
        if let Some(memoized) = self.get_memoized() { memoized }
        else { self.search_matches(tiles) }
    }

    fn search_matches(&self, tiles: &HashMap<u32, Tile>)
    -> std::cell::Ref<'_, [Option<Match>; 4]> {
        // for each border...
        let matches_per_border = self.border.iter().cloned().map(|border| {
            // for each other tile in the set...
            tiles.values().filter(|other_tile| other_tile.id != self.id)
                .zip(std::iter::repeat(border)).flat_map(|(other_tile, border)| 
                    // for each border on the other tile...
                    other_tile.border.iter().enumerate().filter_map(move |(index, &other_border)| {
                        let other_edge = Edge::from_index(index);
                        if border.raw == other_border.raw { 
                            Some(Match { other_tile: other_tile.id, other_edge, flipped: false })
                        } else if border.flipped().raw == other_border.raw {
                            Some(Match { other_tile: other_tile.id, other_edge, flipped: true })
                        } else {
                            None
                        }
                    }
                )
        )});

        // make sure we have no more than 1 match per border
        let mut matches_per_border = matches_per_border.map(|mut matches| {
            let first = matches.next();
            assert!(matches.next().is_none(), "duplicate match");
            first
        });

        let result = [
            matches_per_border.next().unwrap(),
            matches_per_border.next().unwrap(),
            matches_per_border.next().unwrap(),
            matches_per_border.next().unwrap(),
        ];
        assert!(matches_per_border.next().is_none());

        self.memoized_matches.replace(Some(result));
        self.get_memoized().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
struct Match {
    other_tile: u32,
    other_edge: Edge,
    flipped: bool
}
impl Match {
    fn flipped(self) -> Match {
        Match {
            other_tile: self.other_tile,
            other_edge: self.other_edge,
            flipped: !self.flipped
        }
    }
}

pub fn run() {
    let stdin = io::stdin();
    let mut remaining_tiles = stdin.lock().lines()
        .map(|line| line.expect("read error")).collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .filter(|split| !split.is_empty())
        .map(|tile| Tile::parse(&tile).expect("invalid input"))
        .map(|tile| (tile.id, tile))
        .collect::<HashMap<u32, Tile>>();

    let width = (remaining_tiles.len() as f64).sqrt() as usize;
    assert_eq!(width*width, remaining_tiles.len());

    // Arbitrarily declare one corner to be the top left
    let top_left_id = remaining_tiles.values()
        // corners only match two tiles
        .find(|tile| tile.matches(&remaining_tiles).iter().filter(|m| m.is_some()).count() == 2)
        .expect("no corner pieces found").id;
    let mut top_left = remaining_tiles.remove(&top_left_id).unwrap();

    // Rotate it until the rightmost border is occupied
    while top_left.matches(&remaining_tiles)[Edge::Right.index()].is_none() {
        top_left.rotate_cw();
    }
    // Flip it so that the bottom border is occupied
    if top_left.matches(&remaining_tiles)[Edge::Bottom.index()].is_none() {
        top_left.flip_vert();
    }

    let mut grid = Vec::<Tile>::new();
    grid.push(top_left);
    
    while !remaining_tiles.is_empty() {
        // Find the next piece and add it to the grid.
        //
        // Are we advancing rightwards or downwards?
        let horizontal = (grid.len() % width) != 0;
        let prev = if horizontal { grid.last().unwrap() } else { &grid[grid.len()-width] };
        let dir = if horizontal { Edge::Right } else { Edge::Bottom };

        let next_match = prev.matches(&remaining_tiles)[dir.index()].unwrap();
        let mut next = remaining_tiles.remove(&next_match.other_tile)
            .unwrap_or_else(|| panic!("missing tile {}", next_match.other_tile));

        // Orient the new piece correctly.
        let mut edge_on_next =  next_match.other_edge;
        let mut next_flipped = next_match.flipped;
        while edge_on_next != dir.inverse() {
            next.rotate_cw();
            edge_on_next = edge_on_next.advanced(1);
            if [Edge::Top, Edge::Bottom].contains(&edge_on_next) { next_flipped = !next_flipped; }
        }
        if next_flipped {
            if horizontal { next.flip_vert(); }
            else { next.flip_horiz(); }
        }
        assert_eq!(&prev.border[dir.index()].raw, &next.border[dir.inverse().index()].raw);

        grid.push(next);
    }

    // Print the content, for debugging
    for superrow in 0..width { // width and height are the same;
        for subrow in 0..CONTENT_SIZE {
            for tile in grid[superrow*width..(superrow+1)*width].iter() {
                let mut bits = tile.content[subrow];
                for _ in 0..CONTENT_SIZE {
                    bits <<= 1;
                    print!("{}", if bits & (1 << CONTENT_SIZE) != 0 { '#' } else { '.' })
                }
                print!(" ");
            }
            println!();
        }
        println!();
    }

    // Find sea monsters
    const SEA_MONSTER: [&str; 3] = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   "
    ];


    fn find_sea_monster(
        x: usize, y: usize,
        flipped: bool, rotation: i32,
        grid: &[Tile], width: usize)
    -> bool {
        // Look for sea monster at (x, y)
        for (offs_y, monster_row) in SEA_MONSTER.iter().enumerate() {
            for (offs_x, c) in monster_row.bytes().enumerate() {
                let (x, y) = (x as isize, y as isize);
                let (offs_x, offs_y) = (offs_x as isize, offs_y as isize);
                // Transform coordinates
                let (xformed_x, xformed_y) = match rotation {
                    0 => ( (x + offs_x),  (y + offs_y)),
                    1 => ( (y + offs_y), -(x + offs_x)),
                    2 => (-(x + offs_x), -(y + offs_y)),
                    3 => (-(y + offs_y),  (x + offs_x)),
                    _ => unreachable!()
                };
                let xformed_x = if flipped { -xformed_x } else { xformed_x };
                let (xformed_x, xformed_y) = (
                    xformed_x.rem_euclid((CONTENT_SIZE*width) as isize) as usize,
                    xformed_y.rem_euclid((CONTENT_SIZE*width) as isize) as usize
                );

                let (tile_x, tile_y) = (xformed_x / CONTENT_SIZE, xformed_y / CONTENT_SIZE);
                if tile_x > width || tile_y > width { return false; }

                let (cell_x, cell_y) = (xformed_x % CONTENT_SIZE, xformed_y % CONTENT_SIZE);
                let row = grid[tile_y*width + tile_x].content[cell_y];
                let bit = (row >> (CONTENT_SIZE - 1 - cell_x)) & 1;
                if c == b'#' && bit == 0 { return false; }
            }
        }
        true
    }

    let mut monsters = 0;
    for &flipped in [true, false].iter() {
        for rotation in 0..4 {
            for y in 0..(CONTENT_SIZE*width) as usize {
                for x in 0..(CONTENT_SIZE*width) as usize {
                    if find_sea_monster(x, y, flipped, rotation, &grid, width) { monsters += 1; }
                }
            }
        }
    }

    let tiles_per_sea_monster = SEA_MONSTER.iter()
        .map(|s| s.bytes()).flatten()
        .filter(|&c| c == b'#').count();

    let total_tiles = grid.iter()
        .flat_map(|tile| tile.content.iter())
        .map(|word| word.count_ones()).sum::<u32>() as usize;

    println!("{}", total_tiles - monsters*tiles_per_sea_monster);
}
