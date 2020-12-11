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

#[derive(Clone)]
struct Grid<T> {
    cells: Vec<T>,
    width: usize
}

impl<T> Grid<T> {
    fn index(&self, x: usize, y: usize) -> usize {
        y*self.width + x
    }
    fn get(&self, x: usize, y: usize) -> &T {
        self.cells.get(self.index(x, y)).unwrap()
    }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index = self.index(x, y);
        self.cells.get_mut(index).unwrap()
    }

    fn height(&self) -> usize { self.cells.len() / self.width }

    fn empty() -> Grid<T> { Self { cells: Vec::new(), width: 0 } }
}

impl Grid<Cell> {
    /// Runs an iteration. Returns the results, along width
    /// a bool to indicate if we stabalized.
    fn run_iter(self) -> (Self, bool) {
        let mut result = self.clone();
        let mut stable = true;

        for y in 0..self.height() {
            for x in 0..self.width {
                // thanks Duncan
                let neighbors_iter = (-1..=1)
                    .map(move |y| (-1..=1).map(move |x| (x, y)))
                    .flatten()
                    .filter(|x| !matches!(x, (0, 0)));
                let neighbors = neighbors_iter
                    .filter(|(dx, dy)| self.search_seat(x, y, *dx, *dy) == Some(Cell::Occupied))
                    .count();

                let current = result.get_mut(x, y);
                if *current == Cell::Empty && neighbors == 0 {
                    stable = false;
                    *current = Cell::Occupied;
                } else if *current == Cell::Occupied && neighbors >= 5 {
                    stable = false;
                    *current = Cell::Empty;
                }
            }
        }

        (result, stable)
    }

    fn search_seat(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<Cell> {
        let mut x = x as isize;
        let mut y = y as isize;
        loop {
            x += dx;
            y += dy;

            if !(0..self.width as isize).contains(&x) || !(0..self.height() as isize).contains(&y) { return None; }
            match *self.get(x as usize, y as usize) {
                Cell::Floor => (),
                cell => return Some(cell)
            }
        }
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

    let mut stable = false;
    while !stable {
        let (ngrid, nstable) = grid.run_iter();
        grid = ngrid;
        stable = nstable;
    }
    println!("{}", grid.cells.iter().filter(|&&c| c == Cell::Occupied).count());
}
