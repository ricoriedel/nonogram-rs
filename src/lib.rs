//     A fast and lightweight library to solve nonograms.
//     Copyright (C) 2021  Rico Riedel <rico.riedel@protonmail.ch>
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod algo;

#[cfg(feature = "json")]
pub mod json;

use std::ops::{Index, IndexMut};
use std::fmt;

/// A cell of a Nonogram.
#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    /// An unsolved/uninitialized cell.
    Empty,
    /// A filled cell.
    Box,
    /// A "x" cell.
    Space,
}

/// A nonogram, that's it.
#[derive(Clone)]
pub struct Nonogram {
    cols: usize,
    rows: usize,
    data: Vec<Cell>,
}

/// Allows to access a column like an independent object.
///
/// *The length is determined by the row count!*
pub struct ColMut<'a> {
    nonogram: &'a mut Nonogram,
    col: usize,
}

/// Allows to access a row like an independent object.
///
/// *The length is determined by the column count!*
pub struct RowMut<'a> {
    nonogram: &'a mut Nonogram,
    row: usize,
}

/// Trait to access a line of a nonogram like an independent object.
/// This trait is used to reduce code duplication inside the algorithm.
pub trait Line: IndexMut<usize, Output=Cell> {
    fn len(&self) -> usize;
}

impl Nonogram {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            data: vec![Cell::Empty; cols * rows],
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn col_mut(&mut self, col: usize) -> ColMut {
        ColMut {
            nonogram: self,
            col,
        }
    }

    pub fn row_mut(&mut self, row: usize) -> RowMut {
        RowMut {
            nonogram: self,
            row,
        }
    }

    fn index_of(&self, pos: (usize, usize)) -> usize {
        assert!(pos.0 < self.cols);
        assert!(pos.1 < self.rows);

        pos.0 * self.rows + pos.1
    }
}

impl fmt::Display for Nonogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Size: {}x{}", self.cols, self.rows)?;
        writeln!(f)?;

        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", match self[(col, row)] {
                    Cell::Empty => "▒▒",
                    Cell::Box => "██",
                    Cell::Space => "  "
                })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for Nonogram {
    type Output = Cell;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.data[self.index_of(pos)]
    }
}

impl IndexMut<(usize, usize)> for Nonogram {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        let i = self.index_of(pos);

        &mut self.data[i]
    }
}

impl<'a> Index<usize> for ColMut<'a> {
    type Output = Cell;

    fn index(&self, i: usize) -> &Self::Output {
        &self.nonogram[(self.col, i)]
    }
}

impl<'a> IndexMut<usize> for ColMut<'a> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.nonogram[(self.col, i)]
    }
}

impl<'a> Line for ColMut<'a> {
    fn len(&self) -> usize {
        self.nonogram.rows()
    }
}

impl<'a> Index<usize> for RowMut<'a> {
    type Output = Cell;

    fn index(&self, i: usize) -> &Self::Output {
        &self.nonogram[(i, self.row)]
    }
}

impl<'a> IndexMut<usize> for RowMut<'a> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.nonogram[(i, self.row)]
    }
}

impl<'a> Line for RowMut<'a> {
    fn len(&self) -> usize {
        self.nonogram.cols()
    }
}

pub fn solve(cols: Vec<Vec<usize>>, rows: Vec<Vec<usize>>) -> Result<Nonogram, ()> {
    algo::solve(cols, rows)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nonogram_new() {
        let n = Nonogram::new(3, 8);

        assert_eq!(n.cols(), 3);
        assert_eq!(n.rows(), 8);
    }

    #[test]
    fn nonogram_index_mut() {
        let mut n = Nonogram::new(5, 5);

        n[(3, 4)] = Cell::Box;

        assert!(matches!(n[(3, 4)], Cell::Box));
    }

    #[test]
    #[should_panic]
    fn nonogram_index_oob_col() {
        let n = Nonogram::new(3, 8);

        n[(4, 6)];
    }

    #[test]
    #[should_panic]
    fn nonogram_index_oob_row() {
        let n = Nonogram::new(7, 2);

        n[(5, 3)];
    }

    #[test]
    fn nonogram_col_mut_get() {
        let mut n = Nonogram::new(5, 5);

        n[(2, 4)] = Cell::Space;

        assert!(matches!(n.col_mut(2)[4], Cell::Space));
    }

    #[test]
    fn nonogram_col_mut_set() {
        let mut n = Nonogram::new(5, 5);

        n.col_mut(2)[4] = Cell::Space;

        assert!(matches!(n[(2, 4)], Cell::Space));
    }

    #[test]
    fn nonogram_col_mut_len() {
        let mut n = Nonogram::new(2, 7);

        assert_eq!(n.col_mut(0).len(), 7);
    }

    #[test]
    fn nonogram_row_mut_get() {
        let mut n = Nonogram::new(5, 5);

        n[(2, 4)] = Cell::Space;

        assert!(matches!(n.row_mut(4)[2], Cell::Space));
    }

    #[test]
    fn nonogram_row_mut_set() {
        let mut n = Nonogram::new(5, 5);

        n.row_mut(4)[2] = Cell::Space;

        assert!(matches!(n[(2, 4)], Cell::Space));
    }

    #[test]
    fn nonogram_row_mut_len() {
        let mut n = Nonogram::new(7, 2);

        assert_eq!(n.row_mut(0).len(), 7);
    }

    #[test]
    fn nonogram_fmt() {
        let mut n = Nonogram::new(2, 2);

        n[(1, 0)] = Cell::Box;
        n[(1, 1)] = Cell::Box;
        n[(0, 1)] = Cell::Space;

        let str = "Size: 2x2\n\n▒▒██\n  ██\n";

        assert_eq!(format!("{}", n), str);
    }
}