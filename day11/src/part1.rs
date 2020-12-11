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

    fn iter_adjacent(&self, x: usize, y: usize) -> impl Iterator<Item=(usize, usize)> {
        let width = self.width as isize;
        let height = self.height() as isize;
        // thanks Duncan
        (-1..=1)
            .map(move |y| (-1..=1).map(move |x| (x, y)))
            .flatten()
            .filter(|x| !matches!(x, (0, 0)))
            .map(move |(dx, dy)| ((x as isize) + dx, (y as isize) + dy))
            .filter_map(move |(x, y)| {
                if !(0..width).contains(&x) || !(0..height).contains(&y) { None }
                else { Some((x as usize, y as usize)) }
        })
    }
}

impl Grid<Cell> {
    /// Runs an iteration. Returns the results, along width
    /// a bool to indicate if we stabalized.
    fn run_iter(self) -> (Self, bool) {
        let mut result = self.clone();
        let mut stable = true;

        for y in 0..self.height() {
            for x in 0..self.width {
                let neighbors = self.iter_adjacent(x, y)
                    .filter(|(x, y)| self.get(*x, *y) == &Cell::Occupied)
                    .count();
                let current = result.get_mut(x, y);
                if *current == Cell::Empty && neighbors == 0 {
                    stable = false;
                    *current = Cell::Occupied;
                } else if *current == Cell::Occupied && neighbors >= 4 {
                    stable = false;
                    *current = Cell::Empty;
                }
            }
        }

        (result, stable)
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
