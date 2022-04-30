use crate::{Cell, Nonogram};
use std::ops::{Index, IndexMut};

/// A reference to a row or column of a [Nonogram].
/// Used to reduce code duplication.
pub trait LineMut<T>: IndexMut<usize, Output = Cell<T>> {
    fn len(&self) -> usize;
}

/// A reference to a column. See [LineMut].
pub struct ColMut<'a, T> {
    nonogram: &'a mut Nonogram<T>,
    col: usize,
}

impl<'a, T> ColMut<'a, T> {
    pub fn new(nonogram: &'a mut Nonogram<T>, col: usize) -> Self {
        Self { nonogram, col }
    }
}

impl<'a, T> Index<usize> for ColMut<'a, T> {
    type Output = Cell<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nonogram[(self.col, index)]
    }
}

impl<'a, T> IndexMut<usize> for ColMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nonogram[(self.col, index)]
    }
}

impl<'a, T> LineMut<T> for ColMut<'a, T> {
    fn len(&self) -> usize {
        self.nonogram.rows()
    }
}

/// A reference to a row. See [LineMut].
pub struct RowMut<'a, T> {
    nonogram: &'a mut Nonogram<T>,
    row: usize,
}

impl<'a, T> RowMut<'a, T> {
    pub fn new(nonogram: &'a mut Nonogram<T>, row: usize) -> Self {
        Self { nonogram, row }
    }
}

impl<'a, T> Index<usize> for RowMut<'a, T> {
    type Output = Cell<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nonogram[(index, self.row)]
    }
}

impl<'a, T> IndexMut<usize> for RowMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nonogram[(index, self.row)]
    }
}

impl<'a, T> LineMut<T> for RowMut<'a, T> {
    fn len(&self) -> usize {
        self.nonogram.cols()
    }
}

#[cfg(test)]
impl<T> LineMut<T> for Vec<Cell<T>> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn col_mut_index() {
        let n = &mut Nonogram::new(4, 3);

        n[(2, 1)] = Cell::Box { color: 25 };

        let col = ColMut::new(n, 2);

        assert!(matches!(col[1], Cell::Box { color: 25 }));
    }

    #[test]
    fn col_mut_index_mut() {
        let n = &mut Nonogram::new(3, 6);
        {
            let mut col = ColMut::new(n, 1);
            col[4] = Cell::Box { color: 6 };
        }
        assert!(matches!(n[(1, 4)], Cell::Box { color: 6 }));
    }

    #[test]
    fn col_mut_len() {
        let n: &mut Nonogram<()> = &mut Nonogram::new(3, 6);
        let col = ColMut::new(n, 1);

        assert_eq!(6, col.len());
    }

    #[test]
    fn row_mut_index() {
        let n = &mut Nonogram::new(3, 4);

        n[(1, 2)] = Cell::Box { color: 8 };

        let row = RowMut::new(n, 2);

        assert!(matches!(row[1], Cell::Box { color: 8 }));
    }

    #[test]
    fn row_mut_index_mut() {
        let n = &mut Nonogram::new(6, 5);
        {
            let mut row = RowMut::new(n, 4);
            row[1] = Cell::Box { color: 2 };
        }
        assert!(matches!(n[(1, 4)], Cell::Box { color: 2 }));
    }

    #[test]
    fn row_mut_len() {
        let n: &mut Nonogram<()> = &mut Nonogram::new(7, 4);
        let row = RowMut::new(n, 2);

        assert_eq!(7, row.len());
    }
}
