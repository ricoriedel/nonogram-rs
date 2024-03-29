use crate::{Cancelled, Cell, Item, Token};
use collection::Collection;
use grid::Grid;
use rayon::join;

pub mod chain;
pub mod collection;
pub mod grid;
pub mod line;

/// A [super::Cell] that might not has a value yet.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PartCell<T> {
    /// An unknown value.
    Empty,
    /// A box with some color of type `T`.
    Box { color: T },
    /// A space ("x") between chains.
    Space,
}

/// The reason a nonogram could not be solved.
#[derive(Debug)]
pub enum Error {
    /// The supplied data doesn't result in a valid nonogram.
    Invalid,
    /// The collection was full.
    Full,
    /// The operation has been cancelled.
    Cancelled,
}

impl From<Cancelled> for Error {
    fn from(_: Cancelled) -> Self {
        Error::Cancelled
    }
}

impl<T: PartialEq> PartialEq<T> for PartCell<T> {
    fn eq(&self, other: &T) -> bool {
        match self {
            PartCell::Box { color } => color == other,
            _ => false,
        }
    }
}

impl<T> TryFrom<PartCell<T>> for Cell<T> {
    type Error = ();

    fn try_from(value: PartCell<T>) -> Result<Self, Self::Error> {
        match value {
            PartCell::Empty => Err(()),
            PartCell::Box { color } => Ok(Cell::Box { color }),
            PartCell::Space => Ok(Cell::Space),
        }
    }
}

/// A branch which might result in a complete nonogram.
#[derive(Clone)]
pub struct Branch<T> {
    cols: Grid<T>,
    rows: Grid<T>,
}

impl<T: Copy + PartialEq + Send> Branch<T> {
    /// Constructs a new branch from a layout.
    pub fn build(col_grid: Vec<Vec<Item<T>>>, row_grid: Vec<Vec<Item<T>>>) -> Self {
        let col_count = col_grid.len();
        let row_count = row_grid.len();

        let cols = Grid::build(col_grid, row_count);
        let rows = Grid::build(row_grid, col_count);

        Self { cols, rows }
    }

    /// Tries to find the solution to this branch.
    /// Fails if the layout is invalid.
    pub fn solve<TToken: Token>(mut self, collection: &Collection<T, TToken>) {
        match self.try_solve(collection) {
            Ok(_) => match self.find_unsolved() {
                None => {
                    collection.push(self.cols.try_into().unwrap());
                }
                Some(unsolved) => {
                    let (a, b) = self.fork(unsolved);

                    join(|| a.solve(collection), || b.solve(collection));
                }
            },
            Err(_) => (),
        }
    }

    /// Tries to solve a branch without forking.
    fn try_solve<TToken: Token>(&mut self, token: &Collection<T, TToken>) -> Result<(), Error> {
        while self.cols.flagged() || self.rows.flagged() {
            self.cols.update()?;
            self.cols.write_to(&mut self.rows)?;
            self.rows.update()?;
            self.rows.write_to(&mut self.cols)?;

            token.check()?;
        }
        Ok(())
    }

    /// Forks the branch at the given position
    /// with the given color into one with a box and one with a space.
    fn fork(mut self, (col, row, color): (usize, usize, T)) -> (Self, Self) {
        let mut fork = self.clone();

        self.cols.set(col, row, PartCell::Box { color }).unwrap();
        self.rows.set(row, col, PartCell::Box { color }).unwrap();
        fork.cols.set(col, row, PartCell::Space).unwrap();
        fork.rows.set(row, col, PartCell::Space).unwrap();

        (self, fork)
    }

    /// Finds a unsolved cell if there is any.
    ///
    /// Tuple: `(col, row, color)`
    fn find_unsolved(&self) -> Option<(usize, usize, T)> {
        let (cols, rows) = self.cols.len();

        if cols < rows {
            self.cols.find_unsolved()
        } else {
            self.rows
                .find_unsolved()
                .map(|(line, cell, color)| (cell, line, color))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cancel::Cancel;
    use crate::Cell::*;
    use crate::{Solution, Status};

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
        let mut collection = Collection::new(usize::MAX, ());

        Branch::build(cols, rows).solve(&mut collection);

        let solution: Solution<char> = collection.into();
        let nonogram = solution.collection.first().unwrap();

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
        let cols = vec![vec![Item { color: 'a', len: 1 }]];
        let rows = vec![vec![Item { color: 'b', len: 1 }]];

        let mut collection = Collection::new(usize::MAX, ());

        Branch::build(cols, rows).solve(&mut collection);

        let solution: Solution<char> = collection.into();

        assert!(solution.collection.is_empty());
    }

    #[test]
    fn branch_solve_invalid_empty_cols() {
        let cols = vec![];
        let rows = vec![vec![Item { color: 'b', len: 1 }]];

        let mut collection = Collection::new(usize::MAX, ());

        Branch::build(cols, rows).solve(&mut collection);

        let solution: Solution<char> = collection.into();

        assert!(solution.collection.is_empty());
    }

    #[test]
    fn branch_solve_invalid_empty_rows() {
        let cols = vec![vec![Item { color: 'b', len: 1 }]];
        let rows = vec![];

        let mut collection = Collection::new(usize::MAX, ());

        Branch::build(cols, rows).solve(&mut collection);

        let solution: Solution<char> = collection.into();

        assert!(solution.collection.is_empty());
    }

    #[test]
    fn branch_solve_empty() {
        let cols = vec![];
        let rows = vec![];

        let mut collection = Collection::new(usize::MAX, ());

        Branch::build(cols, rows).solve(&mut collection);

        let solution: Solution<char> = collection.into();

        assert_eq!(1, solution.collection.len());
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
        let mut collection = Collection::new(usize::MAX, ());

        Branch::build(cols.clone(), cols).solve(&mut collection);

        let solution: Solution<char> = collection.into();

        assert!(!solution.collection.is_empty());
    }

    #[test]
    fn branch_solve_cancel() {
        let data = vec![
            vec![Item::new('a', 1)],
            vec![Item::new('a', 1)],
            vec![Item::new('a', 1)],
            vec![Item::new('a', 1)],
            vec![Item::new('a', 1)],
        ];
        let mut collection = Collection::new(usize::MAX, Cancel::default());

        Branch::build(data.clone(), data).solve(&mut collection);

        let solution: Solution<char> = collection.into();

        assert!(matches!(solution.status, Status::Cancelled));
    }
}
