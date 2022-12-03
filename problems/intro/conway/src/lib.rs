#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

use std::cmp;

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut grid = Vec::with_capacity(rows * cols);
        for _ in 0..rows * cols {
            grid.push(T::default());
        }
        Self {
            rows,
            cols,
            grid,
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row * self.rows + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.rows + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        for i in self.get_start_index(row)..cmp::min(row + 2, self.rows) {
            for j in self.get_start_index(col)..cmp::min(col + 2, self.cols) {
                if i != row || j != col {
                    neighbours.push((i, j));
                }
            }
        }
        neighbours
    }

    fn get_start_index(&self, index: usize) -> usize {
        return if index == 0 {
            0
        } else {
            index - 1
        };
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self {
            grid
        }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let mut new_grid: Grid<Cell> = Grid::new(self.grid.rows, self.grid.cols);

        for i in 0..self.grid.size().0 {
            for j in 0..self.grid.size().1 {
                match self.grid.get(i, j) {
                    Cell::Alive => {
                        let neighbours = self.grid.neighbours(i, j);
                        let count_alive = self.calc_alive_neighbours(&neighbours);
                        if count_alive < 2 {
                            new_grid.set(Cell::Dead, i, j);
                        } else if count_alive == 2 || count_alive == 3 {
                            new_grid.set(Cell::Alive, i, j);
                        } else if count_alive > 3 {
                            new_grid.set(Cell::Dead, i, j);
                        }
                    }
                    Cell::Dead => {
                        let neighbours = self.grid.neighbours(i, j);
                        let count_alive = self.calc_alive_neighbours(&neighbours);
                        if count_alive == 3 {
                            new_grid.set(Cell::Alive, i, j);
                        }
                    }
                }
            }
        }

        self.grid = new_grid.clone();
    }


    pub fn calc_alive_neighbours(&self, neighbours: &Vec<(usize, usize)>) -> usize {
        let mut count = 0;
        for item in neighbours.iter() {
            match self.grid.get(item.0, item.1) {
                Cell::Alive => count += 1,
                Cell::Dead => {}
            }
        }
        count
    }
}
