use crate::algo::line::Layout;
use crate::{Error, Item};
use crate::line::Line;

/// A grid of numbers used in [super::Branch::try_solve_cols] and [super::Branch::try_solve_rows].
#[derive(Clone)]
pub struct Grid<T> {
    lines: Vec<Layout<T>>
}

impl<T: Copy + PartialEq> Grid<T> {
    /// Constructs a new grid.
    pub fn build(numbers: &Vec<Vec<Item<T>>>, length: usize) -> Self {
        let lines = numbers.iter()
            .map(|col| Layout::build(col, length))
            .collect();

        Self { lines }
    }

    pub fn flagged(&self) -> bool {
        self.lines.iter()
            .map(Layout::flagged)
            .fold(false, |a, b| a | b)
    }

    pub fn flag(&mut self, index: usize) {
        self.lines[index].flag();
    }

    /// Updates a line if it has been flagged as changed.
    pub fn update(&mut self, index: usize, line: &mut impl Line<T>) -> Result<(), Error> {
        let layout = &mut self.lines[index];

        if layout.flagged() {
            layout.clear();
            layout.update(line)?;
            layout.write(line);
        }
        Ok(())
    }

    /// Finds an unsolved chain.
    ///
    /// Tuple: `(color, line, cell)`
    pub fn find_unsolved(&self) -> Option<(T, usize, usize)> {
        for line in 0..self.lines.len() {
            match self.lines[line].find_unsolved() {
                Some((color, cell)) => return Some((color, line, cell)),
                None => (),
            }
        }
        None
    }
}