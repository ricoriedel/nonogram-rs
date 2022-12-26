use std::ops::Range;
use crate::{Cell, Nonogram};

/// A reference to a row or column of a [Nonogram].
/// Used to reduce code duplication.
pub trait Line<T: Copy> {
    fn len(&self) -> usize;

    fn get(&self, index: usize) -> Cell<T>;

    fn set(&mut self, index: usize, value: Cell<T>);

    fn fill(&mut self, range: Range<usize>, value: Cell<T>) {
        for i in range {
            self.set(i, value);
        }
    }
}

/// A reference to a column. See [Line].
pub struct Col<'a, T> {
    nonogram: &'a mut Nonogram<T>,
    col: usize,
}

impl<'a, T> Col<'a, T> {
    pub fn new(nonogram: &'a mut Nonogram<T>, col: usize) -> Self {
        Self { nonogram, col }
    }
}

impl<'a, T: Copy> Line<T> for Col<'a, T> {
    fn len(&self) -> usize {
        self.nonogram.rows()
    }

    fn get(&self, index: usize) -> Cell<T> {
        self.nonogram[(self.col, index)]
    }

    fn set(&mut self, index: usize, value: Cell<T>) {
        self.nonogram[(self.col, index)] = value;
    }
}

/// A reference to a row. See [Line].
pub struct Row<'a, T> {
    nonogram: &'a mut Nonogram<T>,
    row: usize,
}

impl<'a, T> Row<'a, T> {
    pub fn new(nonogram: &'a mut Nonogram<T>, row: usize) -> Self {
        Self { nonogram, row }
    }
}

impl<'a, T: Copy> Line<T> for Row<'a, T> {
    fn len(&self) -> usize {
        self.nonogram.cols()
    }

    fn get(&self, index: usize) -> Cell<T> {
        self.nonogram[(index, self.row)]
    }

    fn set(&mut self, index: usize, value: Cell<T>) {
        self.nonogram[(index, self.row)] = value;
    }
}

#[cfg(test)]
impl<T: Copy> Line<T> for Vec<Cell<T>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Cell<T> {
        self[index]
    }

    fn set(&mut self, index: usize, value: Cell<T>) {
        self[index] = value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn col_mut_index() {
        let n = &mut Nonogram::new(4, 3);

        n[(2, 1)] = Cell::Box { color: 25 };

        let col = Col::new(n, 2);

        assert!(matches!(col.get(1), Cell::Box { color: 25 }));
    }

    #[test]
    fn col_mut_index_mut() {
        let n = &mut Nonogram::new(3, 6);
        {
            let mut col = Col::new(n, 1);
            col.set(4, Cell::Box { color: 6 });
        }
        assert!(matches!(n[(1, 4)], Cell::Box { color: 6 }));
    }

    #[test]
    fn col_mut_len() {
        let n: &mut Nonogram<()> = &mut Nonogram::new(3, 6);
        let col = Col::new(n, 1);

        assert_eq!(6, col.len());
    }

    #[test]
    fn row_mut_index() {
        let n = &mut Nonogram::new(3, 4);

        n[(1, 2)] = Cell::Box { color: 8 };

        let row = Row::new(n, 2);

        assert!(matches!(row.get(1), Cell::Box { color: 8 }));
    }

    #[test]
    fn row_mut_index_mut() {
        let n = &mut Nonogram::new(6, 5);
        {
            let mut row = Row::new(n, 4);
            row.set(1, Cell::Box { color: 2 });
        }
        assert!(matches!(n[(1, 4)], Cell::Box { color: 2 }));
    }

    #[test]
    fn row_mut_len() {
        let n: &mut Nonogram<()> = &mut Nonogram::new(7, 4);
        let row = Row::new(n, 2);

        assert_eq!(7, row.len());
    }
}
