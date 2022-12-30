#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
use serde::de::Error;

use std::ops::{Index, IndexMut};

/// A cell of a [Nonogram].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cell<T> {
    /// A box with some color of type `T`.
    Box { color: T },
    /// A space ("x") between chains.
    Space,
}

/// A nonogram with a fix size containing some [Cell]s.
/// `T` is the type used to represent colors.
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
#[derive(Clone, PartialEq)]
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
            data: vec![Cell::Space; cols * rows],
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

impl<T: Copy> TryFrom<Vec<Vec<Cell<T>>>> for Nonogram<T> {
    type Error = ();

    fn try_from(value: Vec<Vec<Cell<T>>>) -> Result<Self, Self::Error> {
        let row_len = value.len();
        let col_len = value.iter().map(Vec::len).next().unwrap_or(0);

        let mut nonogram = Nonogram::new(col_len, row_len);

        for row in 0..row_len {
            if value[row].len() != col_len {
                return Err(());
            }
            for col in 0..col_len {
                nonogram[(col, row)] = value[row][col];
            }
        }
        Ok(nonogram)
    }
}

impl<T: Copy> From<Nonogram<T>> for Vec<Vec<Cell<T>>> {
    fn from(nonogram: Nonogram<T>) -> Self {
        let mut rows: Vec<Vec<Cell<T>>> = Vec::new();

        for row_i in 0..nonogram.rows() {
            let mut row = Vec::new();

            for col_i in 0..nonogram.cols() {
                row.push(nonogram[(col_i, row_i)]);
            }
            rows.push(row);
        }
        rows
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

#[cfg(feature = "serde")]
impl<T: Copy + Serialize> Serialize for Nonogram<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let data: Vec<Vec<Cell<T>>> = self.clone().into();

        data.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'a, T: Copy + Deserialize<'a>> Deserialize<'a> for Nonogram<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'a>,
    {
        let data: Vec<Vec<Cell<T>>> = Vec::deserialize(deserializer)?;

        data.try_into()
            .map_err(|_| Error::custom("Failed to construct nonogram."))
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

    #[test]
    fn vec_from_nonogram() {
        let mut nonogram = Nonogram::new(2, 3);
        nonogram[(0, 0)] = Cell::Space;
        nonogram[(0, 1)] = Cell::Box { color: 3 };
        nonogram[(0, 2)] = Cell::Space;
        nonogram[(1, 0)] = Cell::Box { color: 4 };
        nonogram[(1, 1)] = Cell::Box { color: 4 };
        nonogram[(1, 2)] = Cell::Space;

        let vec: Vec<Vec<Cell<i32>>> = nonogram.into();

        assert_eq!(3, vec.len());
        assert_eq!(2, vec[0].len());
        assert_eq!(2, vec[0].len());
        assert_eq!(2, vec[0].len());
        assert!(matches!(vec[0][0], Cell::Space));
        assert!(matches!(vec[1][0], Cell::Box { color: 3 }));
        assert!(matches!(vec[2][0], Cell::Space));
        assert!(matches!(vec[0][1], Cell::Box { color: 4 }));
        assert!(matches!(vec[1][1], Cell::Box { color: 4 }));
        assert!(matches!(vec[2][1], Cell::Space));
    }

    #[test]
    fn nonogram_from_vec() {
        let vec = vec![
            vec![Cell::Box { color: 3 }, Cell::Space, Cell::Space],
            vec![Cell::Box { color: 2 }, Cell::Box { color: 5 }, Cell::Space],
        ];

        let nonogram: Nonogram<i32> = vec.try_into().unwrap();

        assert_eq!(3, nonogram.cols());
        assert_eq!(2, nonogram.rows());
        assert!(matches!(nonogram[(0, 0)], Cell::Box { color: 3 }));
        assert!(matches!(nonogram[(1, 0)], Cell::Space));
        assert!(matches!(nonogram[(2, 0)], Cell::Space));
        assert!(matches!(nonogram[(0, 1)], Cell::Box { color: 2 }));
        assert!(matches!(nonogram[(1, 1)], Cell::Box { color: 5 }));
        assert!(matches!(nonogram[(2, 1)], Cell::Space));
    }

    #[test]
    fn serialize_deserialize() {
        let mut src = Nonogram::new(3, 5);
        src[(2, 3)] = Cell::Space;
        src[(1, 0)] = Cell::Box { color: 4 };
        src[(0, 2)] = Cell::Box { color: 2 };

        let json = serde_json::to_string(&src).unwrap();
        let target: Nonogram<i32> = serde_json::from_str(&json).unwrap();

        for col in 0..src.cols() {
            for row in 0..src.rows() {
                assert_eq!(src[(col, row)], target[(col, row)]);
            }
        }
    }
}
