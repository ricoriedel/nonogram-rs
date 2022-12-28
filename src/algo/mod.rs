use crate::{Cell, Error, Item, Nonogram, Token};
use crate::algo::grid::Grid;

pub mod chain;
pub mod line;
mod grid;

/// A branch which might result in a complete nonogram.
#[derive(Clone)]
pub struct Branch<T> {
    cols: Grid<T>,
    rows: Grid<T>,
}

impl<T: Copy + PartialEq> Branch<T> {
    /// Constructs a new branch from a layout.
    pub fn build(col_grid: &Vec<Vec<Item<T>>>, row_grid: &Vec<Vec<Item<T>>>) -> Self {
        let cols = Grid::build(col_grid, row_grid.len());
        let rows = Grid::build(row_grid, col_grid.len());

        Self {
            cols,
            rows,
        }
    }

    /// Tries to find the solution to this branch.
    /// Fails if the layout is invalid.
    pub fn solve(self, token: impl Token) -> Result<Nonogram<T>, Error> {
        // Emulates recursion because there are to many big variables for the stack.

        let mut branches = vec![self];

        while let Some(mut branch) = branches.pop() {
            match branch.try_solve(&token) {
                Ok(_) => {
                    match branch.cols.find_unsolved() {
                        None => return Ok(branch.cols.into()),
                        Some(unsolved) => {
                            let (a, b) = branch.fork(unsolved);

                            branches.push(a);
                            branches.push(b);
                        }
                    }
                },
                Err(Error::Canceled) => return Err(Error::Canceled),
                Err(Error::Invalid) => (),
            }
        }
        Err(Error::Invalid)
    }

    /// Tries to solve a branch without forking.
    fn try_solve(&mut self, token: &impl Token) -> Result<(), Error> {
        while self.cols.flagged() {
            self.cols.update()?;
            self.cols.write_to(&mut self.rows);
            self.rows.update()?;
            self.rows.write_to(&mut self.cols);
            token.check()?;
        }
        Ok(())
    }

    /// Forks the branch at the given position
    /// with the given color into one with a box and one with a space.
    fn fork(mut self, (col, row, color): (usize, usize, T)) -> (Self, Self) {
        let mut fork = self.clone();

        self.cols.set(col, row, Cell::Box { color });
        self.rows.set(row, col, Cell::Box { color });
        fork.cols.set(col, row, Cell::Space);
        fork.rows.set(row, col, Cell::Space);

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
            vec![Item::new('a', 1), Item::new('b', 1)],
            vec![Item::new('b', 1)],
            vec![Item::new('a', 1), Item::new('b', 2)],
        ];
        let rows = vec![
            vec![Item::new('a', 1), Item::new('a', 1)],
            vec![Item::new('b', 3)],
            vec![Item::new('b', 1)],
        ];
        let branch = Branch::build(&cols, &rows);
        let nonogram = branch.solve(()).unwrap();

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
            vec![Item { color: 'a', len: 1 }],
        ];
        let rows = vec![
            vec![Item { color: 'b', len: 1 }],
        ];
        let branch = Branch::build(&cols, &rows);

        assert!(branch.solve(()).is_err());
    }

    #[test]
    fn branch_solve_recursion() {
        let cols = vec![
            vec![Item::new('a', 1)],
            vec![Item::new('a', 1)],
            vec![Item::new('a', 1)],
            vec![Item::new('a', 1)],
            vec![Item::new('a', 1)],
        ];
        let branch = Branch::build(&cols, &cols);

        assert!(branch.solve(()).is_ok());
    }
}