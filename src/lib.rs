pub mod line;

mod algo;

use std::ops::{Index, IndexMut};

/// A cell of a [Nonogram].
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell<T> {
    /// An unknown value.
    Empty,
    /// A box with some color of type [T].
    Box { color: T },
    /// A space ("x") between chains.
    Space,
}

/// A nonogram with a fix size containing some [Cell]s.
/// [T] is the type used to represent colors.
/// ```rust
/// use nonogram_rs::{Nonogram, Cell};
///
/// let mut n: Nonogram<u8> = Nonogram::new(5, 5);
///
/// n[(0, 3)] = Cell::Space;
/// n[(1, 0)] = Cell::Box { color: 0 };
/// n[(4, 2)] = Cell::Box { color: 1 };
///
/// let value = n[(0, 3)];
/// ```
#[derive(Clone, Debug)]
pub struct Nonogram<T> {
    cols: usize,
    rows: usize,
    data: Vec<Cell<T>>,
}

impl<T: Clone> Nonogram<T> {
    /// Constructs a new nonogram.
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            data: vec![Cell::Empty; cols * rows],
        }
    }
}

impl<T> Nonogram<T> {
    /// Returns the column count.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the row count.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the index of a cell by column and row.
    ///
    /// # Panics
    /// If the column or row is out of bounds.
    fn index_of(&self, pos: (usize, usize)) -> usize {
        assert!(pos.0 < self.cols);
        assert!(pos.1 < self.rows);

        pos.1 * self.cols + pos.0
    }
}

impl<T> Index<(usize, usize)> for Nonogram<T> {
    type Output = Cell<T>;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        let index = self.index_of(pos);

        unsafe { self.data.get_unchecked(index) }
    }
}

impl<T> IndexMut<(usize, usize)> for Nonogram<T> {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        let index = self.index_of(pos);

        unsafe { self.data.get_unchecked_mut(index) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nonogram_cols() {
        let n: Nonogram<()> = Nonogram::new(3, 7);

        assert_eq!(3, n.cols());
    }

    #[test]
    fn nonogram_rows() {
        let n: Nonogram<()> = Nonogram::new(5, 2);

        assert_eq!(2, n.rows());
    }

    #[test]
    fn nonogram_index_mut() {
        let mut n = Nonogram::new(5, 2);

        n[(3, 1)] = Cell::Box { color: 5 };

        assert!(matches!(n[(3, 1)], Cell::Box { color: 5 }));
    }

    #[test]
    #[should_panic]
    fn nonogram_index_mut_col_oob() {
        let n: Nonogram<()> = Nonogram::new(4, 8);

        n[(4, 0)];
    }

    #[test]
    #[should_panic]
    fn nonogram_index_mut_row_oob() {
        let n: Nonogram<()> = Nonogram::new(9, 5);

        n[(0, 5)];
    }
}
