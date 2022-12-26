use crate::algo::line::Flags;
use crate::line::{ColMut, RowMut};
use crate::{Cell, Nonogram};
use crate::algo::grid::Grid;

pub mod chain;
pub mod line;
mod grid;

/// A branch which might result in a complete nonogram.
#[derive(Clone)]
pub struct Branch<T> {
    nonogram: Nonogram<T>,
    cols: Grid<T>,
    rows: Grid<T>,
}

impl<T: Copy + PartialEq> Branch<T> {
    /// Constructs a new branch from a layout.
    pub fn new(col_grid: Vec<Vec<(T, usize)>>, row_grid: Vec<Vec<(T, usize)>>) -> Self {
        let nonogram = Nonogram::new(col_grid.len(), row_grid.len());
        let cols = Grid::new(col_grid, nonogram.rows());
        let rows = Grid::new(row_grid, nonogram.cols());

        Self {
            cols,
            rows,
            nonogram
        }
    }

    /// Tries to find the solution to this branch.
    /// Fails if the layout is invalid.
    pub fn solve(self) -> Result<Nonogram<T>, ()> {
        // Emulates recursion because there are to many big variables for the stack.

        let mut branches = vec![self];

        while let Some(mut branch) = branches.pop() {
            match branch.try_solve() {
                Ok(_) => (),
                Err(_) => continue,
            }
            match branch.find_unsolved() {
                None => return Ok(branch.nonogram),
                Some((color, col, row)) => {
                    let (a, b) = branch.fork(color, col, row);

                    branches.push(a);
                    branches.push(b);
                }
            }
        }
        Err(())
    }

    /// Tries to solve a branch without forking.
    fn try_solve(&mut self) -> Result<(), ()> {
        while self.cols.changed() || self.rows.changed() {
            self.try_solve_cols()?;
            self.try_solve_rows()?;
        }
        Ok(())
    }

    /// Tries to solve all columns.
    fn try_solve_cols(&mut self) -> Result<(), ()> {
        self.rows.clear();

        for i in 0..self.rows.len() {
            let line = &mut ColMut::new(&mut self.nonogram, i);

            self.cols.update(i, line, &mut self.rows)?;
        }
        Ok(())
    }

    /// Tries to solve all rows.
    fn try_solve_rows(&mut self) -> Result<(), ()> {
        self.cols.clear();

        for i in 0..self.rows.len() {
            let line = &mut RowMut::new(&mut self.nonogram, i);

            self.rows.update(i, line, &mut self.cols)?;
        }
        Ok(())
    }

    /// Finds and unsolved cell including the only possible color.
    fn find_unsolved(&self) -> Option<(T, usize, usize)> {
        if let Some((color, line, cell)) = self.cols.find_unsolved() {
            return Some((color, line, cell));
        }
        if let Some((color, line, cell)) = self.rows.find_unsolved() {
            return Some((color, cell, line));
        }
        None
    }

    /// Forks the branch at the given position
    /// with the given color into one with a box and one with a space.
    fn fork(mut self, color: T, col: usize, row: usize) -> (Self, Self) {
        self.cols.flag(col);
        self.rows.flag(row);

        let mut fork = self.clone();

        self.nonogram[(col, row)] = Cell::Box { color };
        fork.nonogram[(col, row)] = Cell::Space;

        (self, fork)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Cell::*;

    #[test]
    fn branch_solve() {
        let cols = vec![
            vec![('a', 1), ('b', 1)],
            vec![('b', 1)],
            vec![('a', 1), ('b', 2)],
        ];
        let rows = vec![
            vec![('a', 1), ('a', 1)],
            vec![('b', 3)],
            vec![('b', 1)],
        ];
        let branch = Branch::new(cols, rows);
        let nonogram = branch.solve().unwrap();

        assert!(matches!(nonogram[(0, 0)], Box { color: 'a' }));
        assert!(matches!(nonogram[(1, 0)], Space));
        assert!(matches!(nonogram[(2, 0)], Box { color: 'a' }));

        assert!(matches!(nonogram[(0, 1)], Box { color: 'b' }));
        assert!(matches!(nonogram[(1, 1)], Box { color: 'b' }));
        assert!(matches!(nonogram[(2, 1)], Box { color: 'b' }));

        assert!(matches!(nonogram[(0, 2)], Space));
        assert!(matches!(nonogram[(1, 2)], Space));
        assert!(matches!(nonogram[(2, 2)], Box { color: 'b' }));
    }

    #[test]
    fn branch_solve_invalid() {
        let cols = vec![
            vec![('a', 1)],
        ];
        let rows = vec![
            vec![('b', 1)],
        ];
        let branch = Branch::new(cols, rows);

        assert!(branch.solve().is_err());
    }

    #[test]
    fn branch_solve_recursion() {
        let cols = vec![
            vec![('a', 1)],
            vec![('a', 1)],
            vec![('a', 1)],
            vec![('a', 1)],
            vec![('a', 1)],
        ];
        let rows = vec![
            vec![('a', 1)],
            vec![('a', 1)],
            vec![('a', 1)],
            vec![('a', 1)],
            vec![('a', 1)],
        ];
        let branch = Branch::new(cols, rows);

        assert!(branch.solve().is_ok());
    }
}