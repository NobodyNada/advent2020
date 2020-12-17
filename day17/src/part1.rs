use std::io::{self, BufRead};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    Inactive,
    Active
}

impl Cell {
    fn from_char(c: char) -> Option<Cell> {
        match c {
            '.' => Some(Cell::Inactive),
            '#' => Some(Cell::Active),
            _ => None
        }
    }
}

/// A 3D "infinite" expandable array.
#[derive(Clone)]
struct Grid<T> {
    cells: Vec<T>,
    size: i32     // The size along each axis.
}

impl<T> Grid<T> {
    /// Converts an X/Y index pair to a one-dimensional index in self.cells.
    /// Panics if an index is out of bounds.
    fn index(&self, x: i32, y: i32, z: i32) -> usize {
        let min = -self.size;
        let max = self.size;
        let range = min..max;
        assert!(
            range.contains(&x) &&
            range.contains(&y) &&
            range.contains(&z),
            "indices out of bounds"
        );
        let z = (z-min) as usize;
        let y = (y-min) as usize;
        let x = (x-min) as usize;
        let stride = (self.size*2) as usize;
        ((z*stride) + y)*stride + x
    }

    /// Returns a reference to the element at the given indices.
    /// Panics if an index is out of bounds.
    fn get(&self, x: i32, y: i32, z: i32) -> &T {
        self.cells.get(self.index(x, y, z)).unwrap()
    }
    /// Returns a mutable reference to the element at the gien indices.
    /// Panics if an index is out of bounds.
    fn get_mut(&mut self, x: i32, y: i32, z: i32) -> &mut T {
        let index = self.index(x, y, z);
        self.cells.get_mut(index).unwrap()
    }

    /// Applies an offset to the given x/y coordinate pair
    /// and bounds-checks the result.
    fn offset(&self, xy: (i32, i32, i32), dxy: (i32, i32, i32)) -> Option<(i32, i32, i32)> {
        /// Helper function to compute the one-dimensional offset
        fn offset(x: i32, dx: i32, size: i32) -> Option<i32> {
            let result = x + dx;
            let min = -size;
            let max = size-1;
            if (min..=max).contains(&result) { Some(result) } 
            else { None }
        }
        Some((
            offset(xy.0, dxy.0, self.size)?,
            offset(xy.1, dxy.1, self.size)?,
            offset(xy.2, dxy.2, self.size)?
        ))
    }
}

impl Grid<Cell> {
    /// Expands the grid in each direction.
    fn expand(&mut self, by: i32) {
        let old_size = self.size;
        let new_size = self.size + by;
        assert!(new_size >= old_size);

        let mut result  = Self::new_size(new_size);

        let min = -(self.size);
        let max = self.size - 1;

        for z in min..=max {
            for y in min..=max {
                for x in min..=max {
                    *result.get_mut(x, y, z) = *self.get(x, y, z);
                }
            }
        }
        *self = result;
    }

    fn new_size(size: i32) -> Self {
        Self { 
            cells: std::iter::repeat(Cell::Inactive).take((size*size*size*2*2*2) as usize).collect(),
            size
        }
    }
}

impl Grid<Cell> {
    /// Runs an iteration. Returns false if we updated any
    /// cells, or true if we've stabalized.
    fn run_iter(&mut self) -> bool {
        dbg!("run_iter");
        let original = self.clone();
        let mut stable = true;
        
        // True if any cells on the very edge are active.
        let mut active_boundary = false;

        let min = -(self.size);
        let max = self.size - 1;

        for z in min..=max {
            for y in min..=max {
                for x in min..=max {
                    // By Duncan, iterates from -1..1 in 3 dimensions (excluding the origin)
                    let neighbors_iter = (-1..=1)
                        .flat_map(move |y| (-1..=1).map(move |x| (x, y)))
                        .flat_map(move |(x, y)| (-1..=1).map(move |z| (x, y, z)))
                        .filter(|xyz| !matches!(xyz, (0, 0, 0)))
                        .filter_map(|dxyz| self.offset((x, y, z), dxyz));

                    let active_neighbors = neighbors_iter
                        .filter(|(x, y, z)| (original.get(*x, *y, *z)) == &Cell::Active)
                        .count();
                    //dbg!(active_neighbors);

                    let current = self.get_mut(x, y, z);
                    let new = match (*current, active_neighbors) {
                        (Cell::Inactive, 3) => Cell::Active,
                        (Cell::Active, adjacent) =>
                            if adjacent == 2 || adjacent == 3 { Cell::Active }
                            else { Cell::Inactive },
                        (cell, _) => cell
                    };

                    if *current != new {
                        *current = new;
                        stable = false;
                        if new == Cell::Active && [x, y, z].iter().find(|x| [min, max].contains(x)) != None {
                            active_boundary = true;
                        }
                    }
                }
            }
        }

        if active_boundary { self.expand(1); }

        stable
    }
}

pub fn run() {
    let mut initial_grid = Vec::<Cell>::new();
    let mut width = None;
    for line in io::stdin().lock().lines() {
        let line = line.expect("read error");
        let mut items: Vec<Cell> = line.chars().map(|c| 
            Cell::from_char(c).expect("invalid character in line")
        ).collect();

        assert!(width == None || width == Some(items.len()));
        width = Some(items.len());
        initial_grid.append(&mut items);
    }

    let width = width.expect("no input") as i32;
    let size = (width + 1)/2;
    let mut grid = Grid::<Cell>::new_size(size);
    let min = -size;
    let min_index = grid.index(min, min, 0);
    let max_index = grid.index(min, min, 1);
    grid.cells[min_index..max_index].copy_from_slice(&initial_grid);

    (0..6).for_each(|_| { grid.run_iter(); });
    println!("{}", grid.cells.iter().filter(|&&c| c == Cell::Active).count());
}
