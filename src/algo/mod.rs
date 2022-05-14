use crate::algo::line::{Flags, Layout};
use crate::line::{ColMut, RowMut};
use crate::{Cell, Nonogram};

pub mod chain;
pub mod line;

/// A branch which might result in a complete nonogram.
#[derive(Clone)]
pub struct Branch<T> {
    nonogram: Nonogram<T>,
    cols: Vec<Layout<T>>,
    rows: Vec<Layout<T>>,
}

/// Flag utility used in [Branch::try_solve_cols] and [Branch::try_solve_rows].
struct LayoutFlags<'a, T> {
    changed: bool,
    lines: &'a mut Vec<Layout<T>>
}

impl<'a, T> LayoutFlags<'a, T> {
    /// Constructs a new layout flag util.
    fn new(lines: &'a mut Vec<Layout<T>>) -> Self {
        Self {
            changed: false,
            lines
        }
    }

    /// Returns whether or not any layout was flagged.
    fn changed(&self) -> bool {
        self.changed
    }
}

impl<'a, T> Flags for LayoutFlags<'a, T> {
    fn flag(&mut self, index: usize) {
        self.changed = true;
        self.lines[index].flag();
    }
}

impl<T: Copy + PartialEq> Branch<T> {
    /// Constructs a new branch from a layout.
    pub fn new(col_numbers: Vec<Vec<(T, usize)>>, row_numbers: Vec<Vec<(T, usize)>>) -> Self {
        let nonogram = Nonogram::new(col_numbers.len(), row_numbers.len());
        let cols = col_numbers.into_iter()
            .map(|col| Layout::new(col, nonogram.rows()))
            .collect();
        let rows = row_numbers.into_iter()
            .map(|row| Layout::new(row, nonogram.cols()))
            .collect();

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
        let mut changed = true;

        while changed {
            changed = self.try_solve_cols()?;
            changed |= self.try_solve_rows()?;
        }
        Ok(())
    }

    /// Tries to solve all columns.
    fn try_solve_cols(&mut self) -> Result<bool, ()> {
        let flags = &mut LayoutFlags::new(&mut self.rows);

        for i in 0..self.cols.len() {
            let col = &mut self.cols[i];
            let line = &mut ColMut::new(&mut self.nonogram, i);

            if col.flagged() {
                col.clear();
                col.update(line)?;
                col.write(line, flags);
            }
        }
        Ok(flags.changed())
    }

    /// Tries to solve all rows.
    fn try_solve_rows(&mut self) -> Result<bool, ()> {
        let flags = &mut LayoutFlags::new(&mut self.cols);

        for i in 0..self.rows.len() {
            let row = &mut self.rows[i];
            let line = &mut RowMut::new(&mut self.nonogram, i);

            if row.flagged() {
                row.clear();
                row.update(line)?;
                row.write(line, flags);
            }
        }
        Ok(flags.changed())
    }

    /// Finds and unsolved cell including the only possible color.
    fn find_unsolved(&self) -> Option<(T, usize, usize)> {
        for col in 0..self.cols.len() {
            match self.cols[col].find_unsolved() {
                Some((color, row)) => return Some((color, col, row)),
                None => (),
            }
        }
        None
    }

    /// Forks the branch at the given position
    /// with the given color into one with a box and one with a space.
    fn fork(mut self, color: T, col: usize, row: usize) -> (Self, Self) {
        self.cols[col].flag();
        self.rows[row].flag();

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