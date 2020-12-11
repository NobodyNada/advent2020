use std::io::{self, BufRead};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied
}

impl Cell {
    fn from_char(c: char) -> Option<Cell> {
        match c {
            '.' => Some(Cell::Floor),
            'L' => Some(Cell::Empty),
            '#' => Some(Cell::Occupied),
            _ => None
        }
    }
}

/// A simple 2D array.
#[derive(Clone)]
struct Grid<T> {
    cells: Vec<T>,
    width: usize
}

impl<T> Grid<T> {
    /// Converts an X/Y index pair to a one-dimensional index in self.cells.
    /// Panics if an index is out of bounds.
    fn index(&self, x: usize, y: usize) -> usize {
        assert!(
            (0..self.width()).contains(&x) && 
            (0..self.height()).contains(&y),
            "indices out of bounds"
        );
        y*self.width + x
    }

    /// Returns a reference to the element at the given indices.
    /// Panics if an index is out of bounds.
    fn get(&self, x: usize, y: usize) -> &T {
        self.cells.get(self.index(x, y)).unwrap()
    }
    /// Returns a mutable reference to the element at the gien indices.
    /// Panics if an index is out of bounds.
    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index = self.index(x, y);
        self.cells.get_mut(index).unwrap()
    }

    /// The width of the grid.
    fn width(&self) -> usize { self.width }

    /// The height of the grid.
    fn height(&self) -> usize { self.cells.len() / self.width }

    /// Applies an offset to the given x/y coordinate pair
    /// and bounds-checks the result.
    fn offset(&self, xy: (usize, usize), dxy: (isize, isize)) -> Option<(usize, usize)> {
        /// Helper function to compute the one-dimensional offset
        fn offset(x: usize, dx: isize, max: usize) -> Option<usize> {
            if dx < 0 {
                x.checked_sub(-dx as usize)
            } else {
                x.checked_add(dx as usize)
                    .and_then(|result| if result < max { Some(result) } else { None } )
            }
        }
        Some((
            offset(xy.0, dxy.0, self.width())?,
            offset(xy.1, dxy.1, self.height())?
        ))
    }

    /// Creates a new, empty Grid.
    fn empty() -> Grid<T> { Self { cells: Vec::new(), width: 0 } }
}

impl Grid<Cell> {
    /// Runs an iteration. Returns false if we updated any
    /// cells, or true if we've stabalized.
    fn run_iter(&mut self) -> bool {
        let original = self.clone();
        let mut stable = true;

        for y in 0..self.height() {
            for x in 0..self.width {
                // By Duncan, iterates from -1..1 in 2 dimensions (excluding the origin)
                let neighbors_iter = (-1..=1)
                    .flat_map(move |y| (-1..=1).map(move |x| (x, y)))
                    .filter(|xy| !matches!(xy, (0, 0)));

                let occupied_neighbors = neighbors_iter
                    .filter_map(|dxy| self.offset((x, y), dxy))
                    .filter(|(x, y)| original.get(*x, *y) == &Cell::Occupied)
                    .count();

                let current = self.get_mut(x, y);
                let new = match (*current, occupied_neighbors) {
                    (Cell::Empty, 0) => Cell::Occupied,
                    (Cell::Occupied, adjacent) =>
                        if adjacent < 5 { Cell::Occupied }
                        else { Cell::Empty },
                    (cell, _) => cell
                };

                if *current != new {
                    *current = new;
                    stable = false;
                }
            }
        }

        stable
    }
}

pub fn run() {
    let mut grid = Grid::<Cell>::empty();
    for line in io::stdin().lock().lines() {
        let line = line.expect("read error");
        let mut items: Vec<Cell> = line.chars().map(|c| 
            Cell::from_char(c).expect("invalid character in line")
        ).collect();

        assert!(grid.width == 0 || items.len() == grid.width);
        grid.width = items.len();
        grid.cells.append(&mut items);
    }

    while !grid.run_iter() {}
    println!("{}", grid.cells.iter().filter(|&&c| c == Cell::Occupied).count());
}
