use crate::algo::line::Line;
use crate::{Cell, Error, Item, Nonogram};

/// A group of lines including metadata.
#[derive(Clone)]
pub struct Grid<T> {
    lines: Vec<Line<T>>,
}

impl<T: Copy + PartialEq> Grid<T> {
    /// Constructs a new grid.
    pub fn build(numbers: &Vec<Vec<Item<T>>>, length: usize) -> Self {
        let lines = numbers.iter()
            .map(|col| Line::build(col, length))
            .collect();

        Self { lines }
    }

    /// Returns whether the grid needs to be updated.
    pub fn flagged(&self) -> bool {
        self.lines.iter()
            .map(Line::flagged)
            .fold(false, |a, b| a | b)
    }

    /// Updates the metadata and writes changes.
    pub fn update(&mut self) -> Result<(), Error> {
        for line in self.lines.iter_mut() {
            line.update()?;
        }
        Ok(())
    }

    /// Returns the value of a cell.
    pub fn get(&self, line: usize, cell: usize) -> Cell<T> {
        self.lines[line].get(cell)
    }

    /// Sets the value of a cell.
    ///
    /// Flags the grid, if it has been altered.
    /// See [Grid::flagged].
    pub fn set(&mut self, line: usize, cell: usize, value: Cell<T>) {
        self.lines[line].set(cell, value);
    }

    /// The length of the grid and lines.
    ///
    /// Tuple: `(lines, cells)`
    pub fn len(&self) -> (usize, usize) {
        let inner = self.lines.first().map(Line::len).unwrap_or(0);

        (self.lines.len(), inner)
    }

    /// Copies all values to the **intersecting** grid.
    pub fn write_to(&self, other: &mut Grid<T>) {
        for line in 0..self.lines.len() {
            for cell in 0..self.lines[line].len() {
                other.set(cell, line, self.get(line, cell))
            }
        }
    }

    /// Finds an unsolved chain.
    ///
    /// Tuple: `(color, line, cell)`
    pub fn find_unsolved(&self) -> Option<(usize, usize, T)> {
        for line in 0..self.lines.len() {
            match self.lines[line].find_unsolved() {
                Some((cell, color)) => return Some((line, cell, color)),
                None => (),
            }
        }
        None
    }
}

impl<T: Copy + PartialEq> From<Grid<T>> for Nonogram<T> {
    fn from(grid: Grid<T>) -> Self {
        let (cols, rows) = grid.len();

        let mut nonogram = Nonogram::new(cols, rows);

        for col in 0..cols {
            for row in 0..rows {
                nonogram[(col, row)] = grid.get(col, row);
            }
        }
        nonogram
    }
}