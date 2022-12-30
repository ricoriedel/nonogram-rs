use crate::algo::grid::Grid;
use crate::{Cell, Error, Item, Token};
use collection::Collection;

pub mod chain;
pub mod line;
pub mod collection;

mod grid;

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

impl<T: Copy + PartialEq> Branch<T> {
    /// Constructs a new branch from a layout.
    pub fn build(col_grid: &Vec<Vec<Item<T>>>, row_grid: &Vec<Vec<Item<T>>>) -> Self {
        let cols = Grid::build(col_grid, row_grid.len());
        let rows = Grid::build(row_grid, col_grid.len());

        Self { cols, rows }
    }

    /// Tries to find the solution to this branch.
    /// Fails if the layout is invalid.
    pub fn solve<TToken: Token>(self, collection: &mut Collection<T, TToken>) {
        // Emulates recursion because there are to many big variables for the stack.

        let mut branches = vec![self];

        while let Some(mut branch) = branches.pop() {
            match branch.try_solve(collection) {
                Ok(_) => match branch.find_unsolved() {
                    None => {
                        collection.push(branch.cols.try_into().unwrap());
                    }
                    Some(unsolved) => {
                        let (a, b) = branch.fork(unsolved);

                        branches.push(a);
                        branches.push(b);
                    }
                }
                Err(_) => (),
            }
        }
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

        self.cols.set(col, row, PartCell::Box { color });
        self.rows.set(row, col, PartCell::Box { color });
        fork.cols.set(col, row, PartCell::Space);
        fork.rows.set(row, col, PartCell::Space);

        (self, fork)
    }

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
    use crate::Cell::*;
    use crate::Solution;

    #[derive(Default)]
    struct Cancel;

    impl Token for Cancel {
        fn check(&self) -> Result<(), Error> {
            Err(Error::Cancelled)
        }
    }

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

        Branch::build(&cols, &rows).solve(&mut collection);

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

        Branch::build(&cols, &rows).solve(&mut collection);

        let solution: Solution<char> = collection.into();

        assert!(solution.collection.is_empty());
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

        Branch::build(&cols, &cols).solve(&mut collection);

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

        Branch::build(&data, &data).solve(&mut collection);

        let solution: Solution<char> = collection.into();

        assert!(matches!(solution.error, Some(Error::Cancelled)));
    }
}
